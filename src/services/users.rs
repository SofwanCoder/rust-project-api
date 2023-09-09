use crate::database::ApplicationDatabase;
use crate::helpers;
use crate::helpers::error::AppError;
use crate::models::auth::CreateAuthModel;
use crate::models::user::{CreateUserModel, UpdateUserModel, UserModel};
use crate::repositories::auth::AuthRepository;
use crate::repositories::user::UserRepository;
use crate::types::auths::AuthToken;
use uuid::Uuid;

pub async fn register(
    db: &ApplicationDatabase,
    body: CreateUserModel,
) -> Result<AuthToken, AppError> {
    let connection = &mut db.pg.get_connection();
    let user = UserRepository::create_user(connection, body);

    let auth_session = AuthRepository::create_auth(connection, CreateAuthModel::from(&user));

    let access_token = helpers::token::generate_user_session_access_token(&user, &auth_session)?;
    let refresh_token = helpers::token::generate_user_session_refresh_token(&auth_session)?;

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
