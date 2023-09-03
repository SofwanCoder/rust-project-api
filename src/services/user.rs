use crate::contracts::user::{AuthenticatedUserResponse, CreateUser};
use crate::database::ApplicationDatabase;
use crate::helpers;
use crate::helpers::error::AppError;
use crate::models::user::{CreateUserModel, UserModel};
use crate::repositories::user::UserRepository;
use crate::types::auths::AuthenticatedData;

pub async fn register(
    db: &ApplicationDatabase,
    body: CreateUser,
) -> Result<AuthenticatedUserResponse, AppError> {
    let connection = db.get_connection();
    let user = UserRepository::new(connection).create_user(CreateUserModel {
        id: ulid::Ulid::new().to_string(),
        email: body.email,
        password: body.password,
        name: body.name,
    })?;

    let auth_token = helpers::jwt::encode(AuthenticatedData {
        user_id: user.id.clone(),
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

    Ok(AuthenticatedUserResponse {
        token: auth_token,
        user,
    })
}

pub async fn fetch(db: &ApplicationDatabase, user_id: String) -> Result<UserModel, AppError> {
    let connection = db.get_connection();
    let user = UserRepository::new(connection).find_user(user_id.clone())?;

    if user.is_none() {
        return Err(AppError::new(
            format!("User not found for {}", user_id.clone()),
            crate::helpers::error::AppErrorKind::DatabaseError,
        ));
    }

    Ok(user.unwrap())
}
