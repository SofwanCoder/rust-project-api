use crate::contracts::auth::CreateTokenPayload;
use crate::database::ApplicationDatabase;
use crate::helpers;
use crate::helpers::error::AppError;
use crate::models::auth::CreateAuthModel;
use crate::repositories::auth::AuthRepository;
use crate::repositories::user::UserRepository;
use crate::types::auths::{AuthToken, AuthenticatedData};

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

pub async fn logout(
    db: &ApplicationDatabase,
    auth_data: AuthenticatedData,
) -> Result<(), AppError> {
    let auth_session =
        AuthRepository::find_auth_by_id(&mut db.pg.get_connection(), auth_data.session_id);

    if auth_session.is_none() {
        return Err(AppError::unauthorized("Invalid session"));
    }

    AuthRepository::delete_auth_by_id(&mut db.pg.get_connection(), auth_data.session_id);

    Ok(())
}

pub async fn login_with_password(
    db: &ApplicationDatabase,
    body: CreateTokenPayload,
) -> Result<AuthToken, AppError> {
    let connection = &mut db.pg.get_connection();

    let user = UserRepository::find_by_email(connection, body.email.unwrap());

    if user.is_none() {
        return Err(AppError::unauthorized("Invalid Account or password"));
    }

    let user = user.unwrap();

    let verify_result = helpers::password::verify(user.password.clone(), body.password.unwrap());

    if verify_result.is_err() {
        return Err(AppError::unauthorized("Invalid account or Password"));
    }

    let auth_session = AuthRepository::create_auth(connection, CreateAuthModel::from(&user));

    let access_token = helpers::token::generate_user_session_access_token(&user, &auth_session)?;

    let refresh_token = helpers::token::generate_user_session_refresh_token(&auth_session)?;

    Ok(AuthToken::new(access_token, refresh_token))
}

pub async fn login_with_refresh_token(
    db: &ApplicationDatabase,
    body: CreateTokenPayload,
) -> Result<AuthToken, AppError> {
    let refresh_token = body.refresh_token.unwrap();

    let decoded_token = helpers::token::decode_token_data_for_session(&refresh_token)?;

    let connection = &mut db.pg.get_connection();

    let auth_session = AuthRepository::find_auth_by_id(connection, decoded_token.token_id);

    if auth_session.is_none() {
        return Err(AppError::unauthorized("Refresh token invalid"));
    }

    let auth_session = auth_session.unwrap();

    if auth_session.expires_at.lt(&chrono::Utc::now().naive_utc()) {
        return Err(AppError::unauthorized("Refresh token expired"));
    }

    let user = UserRepository::find_user_by_id(connection, decoded_token.user_id);

    if user.is_none() {
        return Err(AppError::unauthorized("Referenced user does not exist"));
    }

    let user = user.unwrap();

    let access_token = helpers::token::generate_user_session_access_token(&user, &auth_session)?;

    Ok(AuthToken::new(access_token, refresh_token))
}
