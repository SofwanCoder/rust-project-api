use crate::contracts::user::CreateUserPayload;
use crate::database::pg::ApplicationPgDatabase;
use crate::helpers;
use crate::helpers::error::AppError;
use crate::models::auth::CreateAuthModel;
use crate::models::user::{CreateUserModel, UserModel};
use crate::repositories::auth::AuthRepository;
use crate::repositories::user::UserRepository;
use crate::types::auths::AuthToken;
use crate::utilities::rand::generate_uuid;
use uuid::Uuid;

pub async fn register(
    db: &ApplicationPgDatabase,
    body: CreateUserPayload,
) -> Result<AuthToken, AppError> {
    let connection = &mut db.get_connection();
    let user = UserRepository::create_user(
        connection,
        CreateUserModel {
            id: generate_uuid(),
            email: body.email,
            password: helpers::password::hash(body.password)?,
            name: body.name,
        },
    );

    let auth_session = AuthRepository::create_auth(connection, CreateAuthModel::from(&user));

    let access_token = helpers::token::generate_user_session_access_token(&user, &auth_session)?;
    let refresh_token = helpers::token::generate_user_session_refresh_token(&auth_session)?;

    Ok(AuthToken::new(access_token, refresh_token))
}

pub async fn fetch(db: &ApplicationPgDatabase, user_id: Uuid) -> Result<UserModel, AppError> {
    let connection = &mut db.get_connection();
    let user = UserRepository::find_user_by_id(connection, user_id);

    if user.is_none() {
        return Err(AppError::not_found(format!(
            "User not found for {}",
            user_id.clone()
        )));
    }

    Ok(user.unwrap())
}
