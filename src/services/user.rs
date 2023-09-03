use crate::contracts::user::CreateUserPayload;
use crate::database::ApplicationDatabase;
use crate::helpers;
use crate::helpers::error::AppError;
use crate::models::user::{CreateUserModel, UserModel};
use crate::repositories::user::UserRepository;
use crate::types::auths::{AuthToken, AuthenticatedData};
use crate::types::user::UserWithAuthInfo;
use crate::utilities::rand::generate_uuid;
use uuid::Uuid;

pub async fn register(
    db: &ApplicationDatabase,
    body: CreateUserPayload,
) -> Result<UserWithAuthInfo, AppError> {
    let connection = db.get_connection();
    let user = UserRepository::new(connection).create_user(CreateUserModel {
        id: generate_uuid(),
        email: body.email,
        password: helpers::password::hash(body.password.as_bytes())?,
        name: body.name,
    })?;

    let auth_token = helpers::jwt::encode(AuthenticatedData {
        user_id: user.id,
        clearance_level: 1,
        ..AuthenticatedData::default()
    })
    .map_err(|err| {
        log::error!("Error: {:?}", err);
        AppError::new(
            "Error creating user".to_string(),
            crate::helpers::error::AppErrorKind::DatabaseError,
        )
    })?;

    Ok(UserWithAuthInfo {
        authentication: AuthToken::new(auth_token.clone(), auth_token.clone()),
        user,
    })
}

pub async fn fetch(db: &ApplicationDatabase, user_id: Uuid) -> Result<UserModel, AppError> {
    let connection = db.get_connection();
    let user = UserRepository::new(connection).find_user_by_id(user_id)?;

    if user.is_none() {
        return Err(AppError::new(
            format!("User not found for {}", user_id.clone()),
            crate::helpers::error::AppErrorKind::DatabaseError,
        ));
    }

    Ok(user.unwrap())
}
