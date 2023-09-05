use crate::contracts::auth::CreateTokenPayload;
use crate::database::ApplicationDatabase;
use crate::helpers;
use crate::helpers::error::AppError;
use crate::repositories::user::UserRepository;
use crate::types::auths::{AuthToken, AuthenticatedData};

pub async fn login(
    db: &ApplicationDatabase,
    body: CreateTokenPayload,
) -> Result<AuthToken, AppError> {
    let connection = db.get_connection();

    let user = UserRepository::new(connection).find_by_email(body.email)?;

    if user.is_none() {
        return Err(AppError::new(
            "Invalid Account or password".to_string(),
            helpers::error::AppErrorKind::AuthorizationError,
        ));
    }

    let user = user.unwrap();

    let verify_result = helpers::password::verify(user.password, body.password);

    if verify_result.is_err() {
        return Err(AppError::new(
            "Invalid account or Password".to_string(),
            helpers::error::AppErrorKind::AuthorizationError,
        ));
    }

    let auth_token = helpers::jwt::encode(AuthenticatedData {
        user_id: user.id,
        clearance_level: 1,
        ..AuthenticatedData::default()
    })
    .map_err(|err| {
        log::error!("Error: {:?}", err);
        AppError::new(
            "Error requesting auth token".to_string(),
            helpers::error::AppErrorKind::InternalError,
        )
    })?;

    Ok(AuthToken::new(auth_token.clone(), auth_token.clone()))
}
