use crate::database::ApplicationDatabase;
use crate::events::AppEvent;
use crate::helpers::error::AppError;
use crate::models::auth::CreateAuthModel;
use crate::models::user::{CreateUserModel, UpdatePasswordModel, UpdateUserModel, UserModel};
use crate::repositories::auth::AuthRepository;
use crate::repositories::user::UserRepository;
use crate::types::auths::AuthToken;
use crate::{helpers, ApplicationContext};
use uuid::Uuid;

pub async fn register(
    ctx: &ApplicationContext,
    body: CreateUserModel,
) -> Result<AuthToken, AppError> {
    let connection = &mut ctx.db.pg.get_connection();
    let user = UserRepository::create_user(connection, body);

    let auth_session = AuthRepository::create_auth(connection, CreateAuthModel::from(&user));

    let access_token = helpers::token::generate_user_session_access_token(&user, &auth_session)?;
    let refresh_token = helpers::token::generate_user_session_refresh_token(&auth_session)?;

    crate::events::users::UserRegistered::new(user.id, user.name, user.email)
        .publish(&ctx.db.ampq.get_connection().await?)
        .await?;

    Ok(AuthToken::new(access_token, refresh_token))
}

pub async fn fetch_user(db: &ApplicationDatabase, user_id: Uuid) -> Result<UserModel, AppError> {
    let connection = &mut db.pg.get_connection();
    let user = UserRepository::find_user_by_id(connection, user_id);

    if user.is_none() {
        return Err(AppError::not_found(format!(
            "User not found for {}",
            user_id.clone()
        )));
    }

    Ok(user.unwrap())
}

pub async fn fetch_users(db: &ApplicationDatabase) -> Result<Vec<UserModel>, AppError> {
    let connection = &mut db.pg.get_connection();
    let users = UserRepository::find_users(connection);

    Ok(users)
}

pub async fn update_user(
    db: &ApplicationDatabase,
    user_id: Uuid,
    data: UpdateUserModel,
) -> Result<UserModel, AppError> {
    let connection = &mut db.pg.get_connection();
    let user = UserRepository::find_user_by_id(connection, user_id);

    if user.is_none() {
        return Err(AppError::not_found(format!(
            "User not found for {}",
            user_id.clone()
        )));
    }

    let user = UserRepository::update_user(connection, user_id, data);

    Ok(user)
}

pub async fn update_password(
    db: &ApplicationDatabase,
    user_id: Uuid,
    data: UpdatePasswordModel,
) -> Result<UserModel, AppError> {
    let connection = &mut db.pg.get_connection();
    let user = UserRepository::find_user_by_id(connection, user_id);

    if user.is_none() {
        return Err(AppError::not_found(format!(
            "User not found for {}",
            user_id.clone()
        )));
    }

    let user = user.unwrap();

    helpers::password::verify(user.password, data.current_password)
        .map_err(|_| AppError::unauthorized("Invalid account or Password"))?;

    let user = UserRepository::update_user(
        connection,
        user_id,
        UpdateUserModel {
            password: Some(data.new_password),
            ..Default::default()
        },
    );

    Ok(user.into())
}
