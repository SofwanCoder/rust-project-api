use crate::{
    contracts::user_contract::UpdateUserPayload,
    database::ApplicationDatabase,
    events::{user::password_changed::UserPasswordChanged, AppEvent},
    helpers,
    helpers::error::AppError,
    models::user::Model as UserModel,
    repositories::{auth::AuthRepository, user::UserRepository},
    types::{
        auth_types::{AuthToken, CreateAuthModel},
        user_types::{CreateUser, UpdatePassword, UpdateUser},
    },
    ApplicationContext,
};
use sea_orm::TransactionTrait;
use tracing::{debug, instrument, trace};
use uuid::Uuid;

#[instrument]
pub async fn register_a_user(
    ctx: &ApplicationContext,
    body: CreateUser,
) -> Result<AuthToken, AppError> {
    debug!("Registering a user");
    let connection = &mut ctx.db.source.get_connection().await?;
    let transaction_result = connection
        .transaction(|txn| {
            Box::pin(async move {
                debug!("Requesting user creation");
                let user = UserRepository::create_user(txn, body).await?;

                debug!("Requesting auth session creation");
                let auth_session =
                    AuthRepository::create_auth(txn, CreateAuthModel::from(&user)).await;

                if auth_session.is_err() {
                    return Err(AppError::internal_server(
                        "Error occurred while creating auth session",
                    ));
                }

                let auth_session = auth_session.unwrap();

                Ok((user, auth_session))
            })
        })
        .await
        .map_err(|_| {
            debug!("Error occurred while creating user");
            AppError::internal_server("Error occurred while creating user")
        })?;

    let (user, auth_session) = transaction_result;

    debug!("Generating access token");
    let access_token = helpers::token::generate_user_session_access_token(&user, &auth_session)?;
    debug!("Generating refresh token");
    let refresh_token = helpers::token::generate_user_session_refresh_token(&auth_session)?;

    debug!("Publishing user registered event");
    crate::events::user::registered::UserRegistered::new(user.id, user.name, user.email)
        .publish(&ctx.db.ampq.get_connection().await?)
        .await?;

    Ok(AuthToken::new(access_token, refresh_token))
}

#[instrument]
pub async fn fetch_a_user(ctx: &ApplicationContext, user_id: Uuid) -> Result<UserModel, AppError> {
    let connection = &mut ctx.db.source.get_connection().await?;
    let user = UserRepository::find_user_by_id(connection, user_id)
        .await
        .ok_or_else(|| {
            trace!("User not found for {}", user_id);
            AppError::not_found(format!("User not found for {}", user_id))
        })?;

    Ok(user)
}

#[instrument]
pub async fn fetch_some_users(db: &ApplicationDatabase) -> Result<Vec<UserModel>, AppError> {
    debug!("Fetching some users");
    let connection = &mut db.source.get_connection().await?;
    let users = UserRepository::find_users(connection).await;

    Ok(users)
}

#[instrument]
pub async fn update_a_user(
    db: &ApplicationDatabase,
    user_id: Uuid,
    data: UpdateUserPayload,
) -> Result<UserModel, AppError> {
    debug!("Updating a user with id: {}", user_id);
    let connection = &mut db.source.get_connection().await?;
    UserRepository::find_user_by_id(connection, user_id)
        .await
        .ok_or_else(|| {
            trace!("User not found for {}", user_id);
            AppError::not_found("User not found")
        })?;

    debug!("Updating the user");
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

#[instrument(skip_all)]
pub async fn update_a_user_password(
    db: &ApplicationDatabase,
    user_id: Uuid,
    data: UpdatePassword,
) -> Result<UserModel, AppError> {
    debug!("Updating a user password with id: {}", user_id);
    let connection = &mut db.source.get_connection().await?;
    let user = UserRepository::find_user_by_id(connection, user_id)
        .await
        .ok_or_else(|| {
            trace!("User not found for {}", user_id);
            AppError::not_found("User not found")
        })?;

    debug!("Verifying current password");
    helpers::password::verify_password(user.password, data.current_password)
        .map_err(|_| AppError::unauthorized("Invalid password"))?;

    debug!("Updating user password");
    let user = UserRepository::update_user(
        connection,
        user_id,
        UpdateUser {
            password: Some(data.new_password),
            ..Default::default()
        },
    )
    .await?;

    debug!("Publishing user password changed event");
    UserPasswordChanged::new(user.id.clone(), user.name.clone(), user.email.clone())
        .publish(&db.ampq.get_connection().await?)
        .await?;

    Ok(user)
}
