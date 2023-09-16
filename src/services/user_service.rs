use crate::{
    contracts::user_contract::{UpdatePasswordPayload, UpdateUserPayload},
    database::ApplicationDatabase,
    events::AppEvent,
    helpers,
    helpers::error_helper::AppError,
    models::user::Model as UserModel,
    repositories::{auth_repository::AuthRepository, user_repository::UserRepository},
    types::{
        auth_types::{AuthToken, CreateAuthModel},
        user_types::CreateUser,
    },
    ApplicationContext,
};
use sea_orm::TransactionTrait;
use tracing::instrument;
use uuid::Uuid;

#[instrument]
pub async fn register(ctx: &ApplicationContext, body: CreateUser) -> Result<AuthToken, AppError> {
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

    let transaction_result = transaction_result.unwrap();

    let (user, auth_session) = transaction_result;

    let access_token =
        helpers::token_helper::generate_user_session_access_token(&user, &auth_session)?;
    let refresh_token = helpers::token_helper::generate_user_session_refresh_token(&auth_session)?;

    crate::events::user::user_registered::UserRegistered::new(user.id, user.name, user.email)
        .publish(&ctx.db.ampq.get_connection().await?)
        .await?;

    Ok(AuthToken::new(access_token, refresh_token))
}

#[instrument]
pub async fn fetch_user(ctx: &ApplicationContext, user_id: Uuid) -> Result<UserModel, AppError> {
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
pub async fn fetch_users(db: &ApplicationDatabase) -> Result<Vec<UserModel>, AppError> {
    let connection = &mut db.postgres.get_connection().await?;
    let users = UserRepository::find_users(connection).await;

    Ok(users)
}

#[instrument]
pub async fn update_user(
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

    let user = UserRepository::update_user(connection, user_id, data).await;

    Ok(user)
}

#[instrument]
pub async fn update_password(
    db: &ApplicationDatabase,
    user_id: Uuid,
    data: UpdatePasswordPayload,
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
        .map_err(|_| AppError::unauthorized("Invalid account or Password"))?;

    // let user = UserRepository::update_user(
    //     connection,
    //     user_id,
    //     UpdatePasswordPayload {
    //         password: Some(data.new_password),
    //         ..Default::default()
    //     },
    // );

    Ok(user)
}
