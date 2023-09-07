use crate::contracts::auth::CreateTokenPayload;
use crate::database::ApplicationDatabase;
use crate::helpers;
use crate::helpers::error::AppError;
use crate::models::auth::CreateAuthModel;
use crate::repositories::auth::AuthRepository;
use crate::repositories::user::UserRepository;
use crate::types::auths::AuthToken;
use crate::utilities::rand::generate_uuid;

pub async fn login(
    db: &ApplicationDatabase,
    body: CreateTokenPayload,
) -> Result<AuthToken, AppError> {
    match body.grant_type.as_str() {
        "password" => login_with_password(db, body).await,
        "refresh_token" => login_with_refresh_token(db, body).await,
        _ => panic!("We should never get here| Invalid grant type"),
    }
}

pub async fn login_with_password(
    db: &ApplicationDatabase,
    body: CreateTokenPayload,
) -> Result<AuthToken, AppError> {
    let connection = &mut db.get_connection();

    let (user, connection) = UserRepository::find_by_email(connection, body.email.unwrap());

    if user.is_none() {
        return Err(AppError::new(
            "Invalid Account or password".to_string(),
            helpers::error::AppErrorKind::AuthorizationError,
        ));
    }

    let user = user.unwrap();

    let verify_result = helpers::password::verify(user.password.clone(), body.password.unwrap());

    if verify_result.is_err() {
        return Err(AppError::new(
            "Invalid account or Password".to_string(),
            helpers::error::AppErrorKind::AuthorizationError,
        ));
    }

    let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::days(30);

    let (auth_session, _) = AuthRepository::create_auth(
        connection,
        CreateAuthModel {
            id: generate_uuid(),
            user_id: user.id,
            expires_at,
        },
    );

    let access_token = helpers::token::generate_user_access_token(&user)?;

    let refresh_token = helpers::token::generate_token_data_for_session(&auth_session)?;

    Ok(AuthToken::new(access_token.clone(), refresh_token.clone()))
}

pub async fn login_with_refresh_token(
    db: &ApplicationDatabase,
    body: CreateTokenPayload,
) -> Result<AuthToken, AppError> {
    let refresh_token = body.refresh_token.unwrap();

    let decoded_token = helpers::token::decode_token_data_for_session(&refresh_token)?;

    let connection = &mut db.get_connection();

    let (auth_session, connection) =
        AuthRepository::find_auth_by_id(connection, decoded_token.token_id);

    if auth_session.is_none() {
        return Err(AppError::unauthorized("Refresh token invalid".to_string()));
    }

    let auth_session = auth_session.unwrap();

    if auth_session.expires_at.lt(&chrono::Utc::now().naive_utc()) {
        return Err(AppError::unauthorized("Refresh token expired".to_string()));
    }

    let (user, _) = UserRepository::find_user_by_id(connection, decoded_token.user_id);

    if user.is_none() {
        return Err(AppError::unauthorized(
            "Referenced user does not exist".to_string(),
        ));
    }

    let user = user.unwrap();

    let access_token = helpers::token::generate_user_access_token(&user)?;

    Ok(AuthToken::new(access_token, refresh_token))
}
