use crate::{
    contracts::auth_contract::CreateTokenPayload,
    database::ApplicationDatabase,
    helpers,
    helpers::error_helper::AppError,
    repositories::{auth_repository::AuthRepository, user_repository::UserRepository},
    types::auth_types::{AuthToken, AuthenticatedData, CreateAuthModel},
};

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
    let connection = &mut db.source.get_connection().await?;
    let auth_session = AuthRepository::find_auth_by_id(connection, auth_data.session_id).await;

    if auth_session.is_none() {
        return Err(AppError::unauthorized("Invalid session"));
    }

    AuthRepository::delete_auth_by_id(connection, auth_data.session_id).await;

    Ok(())
}

pub async fn login_with_password(
    db: &ApplicationDatabase,
    body: CreateTokenPayload,
) -> Result<AuthToken, AppError> {
    let connection = &mut db.source.get_connection().await?;

    let user = UserRepository::find_by_email(connection, body.email.unwrap()).await;

    if user.is_none() {
        return Err(AppError::unauthorized("Invalid Account or password"));
    }

    let user = user.unwrap();

    helpers::password_helper::verify(user.password.clone(), body.password.unwrap())
        .map_err(|_| AppError::unauthorized("Invalid account or Password"))?;

    let auth_session =
        AuthRepository::create_auth(connection, CreateAuthModel::from(&user)).await?;

    let access_token =
        helpers::token_helper::generate_user_session_access_token(&user, &auth_session)?;

    let refresh_token = helpers::token_helper::generate_user_session_refresh_token(&auth_session)?;

    Ok(AuthToken::new(access_token, refresh_token))
}

pub async fn login_with_refresh_token(
    db: &ApplicationDatabase,
    body: CreateTokenPayload,
) -> Result<AuthToken, AppError> {
    let refresh_token = body.refresh_token.unwrap();

    let decoded_token = helpers::token_helper::decode_token_data_for_session(&refresh_token)?;

    let connection = &mut db.source.get_connection().await?;

    let auth_session = AuthRepository::find_auth_by_id(connection, decoded_token.token_id).await;

    if auth_session.is_none() {
        return Err(AppError::unauthorized("Refresh token invalid"));
    }

    let auth_session = auth_session.unwrap();

    if auth_session.expires_at.lt(&chrono::Utc::now().naive_utc()) {
        return Err(AppError::unauthorized("Refresh token expired"));
    }

    let user = UserRepository::find_user_by_id(connection, decoded_token.user_id).await;

    if user.is_none() {
        return Err(AppError::unauthorized("Referenced user does not exist"));
    }

    let user = user.unwrap();

    let access_token =
        helpers::token_helper::generate_user_session_access_token(&user, &auth_session)?;

    Ok(AuthToken::new(access_token, refresh_token))
}
