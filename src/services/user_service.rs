use crate::{
    contracts::user_contract::UpdateUserPayload,
    database::ApplicationDatabase,
    events::{user::password_changed::UserPasswordChanged, AppEvent},
    helpers,
    helpers::error_helper::AppError,
    models::user::Model as UserModel,
    repositories::{auth_repository::AuthRepository, user_repository::UserRepository},
    types::{
        auth_types::{AuthToken, CreateAuthModel},
        user_types::{CreateUser, UpdatePassword, UpdateUser},
    },
    ApplicationContext,
};
use sea_orm::TransactionTrait;
use tracing::instrument;
use uuid::Uuid;

#[instrument]
pub async fn register_a_user(
    ctx: &ApplicationContext,
    body: CreateUser,
) -> Result<AuthToken, AppError> {
    let connection = &mut ctx.db.postgres.get_connection().await?;
    let transaction_result = connection
        .transaction(|txn| {
            Box::pin(async move {
                let user = UserRepository::create_user(txn, body).await?;

                let auth_session =
                    AuthRepository::create_auth(txn, CreateAuthModel::from(&user)).await;

                if auth_session.is_err() {
                    return Err(AppError::internal_server(
                        "Error occurred while creating auth session",
                    ));
                }

                Ok((user, auth_session.unwrap()))
            })
        })
        .await;

    if transaction_result.is_err() {
        return Err(AppError::internal_server("Unknown error"));
    }

    let (user, auth_session) = transaction_result.unwrap();

    let access_token =
        helpers::token_helper::generate_user_session_access_token(&user, &auth_session)?;
    let refresh_token = helpers::token_helper::generate_user_session_refresh_token(&auth_session)?;

    crate::events::user::registered::UserRegistered::new(user.id, user.name, user.email)
        .publish(&ctx.db.ampq.get_connection().await?)
        .await?;

    Ok(AuthToken::new(access_token, refresh_token))
}

#[instrument]
pub async fn fetch_a_user(ctx: &ApplicationContext, user_id: Uuid) -> Result<UserModel, AppError> {
    let connection = &mut ctx.db.postgres.get_connection().await?;
    let user = UserRepository::find_user_by_id(connection, user_id).await;

    if user.is_none() {
        return Err(AppError::not_found(format!(
            "User not found for {}",
            user_id.clone()
        )));
    }

    Ok(user.unwrap())
}

#[instrument]
pub async fn fetch_some_users(db: &ApplicationDatabase) -> Result<Vec<UserModel>, AppError> {
    let connection = &mut db.postgres.get_connection().await?;
    let users = UserRepository::find_users(connection).await;

    Ok(users)
}

#[instrument]
pub async fn update_a_user(
    db: &ApplicationDatabase,
    user_id: Uuid,
    data: UpdateUserPayload,
) -> Result<UserModel, AppError> {
    let connection = &mut db.postgres.get_connection().await?;
    let user = UserRepository::find_user_by_id(connection, user_id).await;

    if user.is_none() {
        return Err(AppError::not_found(format!(
            "User not found for {}",
            user_id.clone()
        )));
    }

    let user = UserRepository::update_user(
        connection,
        user_id,
        UpdateUser {
            name: data.name,
            email: data.email,
            ..Default::default()
        },
    )
    .await?;

    Ok(user)
}

#[instrument]
pub async fn update_a_user_password(
    db: &ApplicationDatabase,
    user_id: Uuid,
    data: UpdatePassword,
) -> Result<UserModel, AppError> {
    let connection = &mut db.postgres.get_connection().await?;
    let user = UserRepository::find_user_by_id(connection, user_id).await;

    if user.is_none() {
        return Err(AppError::not_found(format!(
            "User not found for {}",
            user_id.clone()
        )));
    }

    let user = user.unwrap();

    helpers::password_helper::verify(user.password.clone(), data.current_password.clone())
        .map_err(|_| AppError::unauthorized("Invalid password"))?;

    let user = UserRepository::update_user(
        connection,
        user_id,
        UpdateUser {
            password: Some(data.new_password),
            ..Default::default()
        },
    )
    .await?;

    UserPasswordChanged::new(user.id.clone(), user.name.clone(), user.email.clone())
        .publish(&db.ampq.get_connection().await?)
        .await?;

    Ok(user)
}
