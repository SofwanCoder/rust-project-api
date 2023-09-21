use crate::{
    contracts_::auth_contract::{CreateTokenPayload, GrantType},
    database::ApplicationDatabase,
    helpers,
    helpers::error::AppError,
    repositories::{auth::AuthRepository, user::UserRepository},
    types_::auth_types::{AuthToken, AuthenticatedData, CreateAuthModel},
};
use tracing::{debug, error, field::debug, instrument};

#[instrument(skip_all)]
pub async fn login_a_user(
    db: &ApplicationDatabase,
    body: CreateTokenPayload,
) -> Result<AuthToken, AppError> {
    debug!("Login a user with grant type: {}", body.grant_type);
    match body.grant_type {
        GrantType::Password => login_with_password(db, body).await,
        GrantType::RefreshToken => login_with_refresh_token(db, body).await,
        _ => {
            error!("Invalid grant type: {}", body.grant_type);
            panic!("We should never get here| Invalid grant type");
        }
    }
}

#[instrument(skip_all)]
pub async fn logout_a_user(
    db: &ApplicationDatabase,
    auth_data: AuthenticatedData,
) -> Result<(), AppError> {
    let connection = &mut db.source.get_connection().await?;
    debug!("Finding auth session by id");
    AuthRepository::find_auth_by_id(connection, auth_data.session_id)
        .await
        .ok_or_else(|| {
            debug!("Invalid session id or session expired");
            AppError::unauthorized("Invalid session id or session expired")
        })?;

    debug!("Deleting auth session");
    AuthRepository::delete_auth_by_id(connection, auth_data.session_id).await?;

    Ok(())
}

#[instrument(skip_all)]
pub async fn login_with_password(
    db: &ApplicationDatabase,
    body: CreateTokenPayload,
) -> Result<AuthToken, AppError> {
    debug!("Login with password");
    let connection = &mut db.source.get_connection().await?;

    debug!("Finding user by email");
    let user = UserRepository::find_by_email(connection, body.email.unwrap())
        .await
        .ok_or(AppError::unauthorized("Invalid Account or password"))?;

    debug!("Verifying password");
    helpers::password::verify_password(user.password.clone(), body.password.unwrap())
        .map_err(|_| AppError::unauthorized("Invalid account or Password"))?;

    debug!("Creating auth session");
    let auth_session =
        AuthRepository::create_auth(connection, CreateAuthModel::from(&user)).await?;

    debug!("Generating access token");
    let access_token = helpers::token::generate_user_session_access_token(&user, &auth_session)?;

    debug!("Generating refresh token");
    let refresh_token = helpers::token::generate_user_session_refresh_token(&auth_session)?;

    debug("Login Complete with password");
    Ok(AuthToken::new(access_token, refresh_token))
}

#[instrument(skip_all)]
pub async fn login_with_refresh_token(
    db: &ApplicationDatabase,
    body: CreateTokenPayload,
) -> Result<AuthToken, AppError> {
    debug!("Login with refresh token");
    let refresh_token = body.refresh_token.unwrap();

    let decoded_token = helpers::token::decode_token_data_for_session(&refresh_token)?;

    let connection = &mut db.source.get_connection().await?;

    debug!("Finding auth session by id");
    let auth_session = AuthRepository::find_auth_by_id(connection, decoded_token.token_id)
        .await
        .ok_or_else(|| {
            debug!("Invalid session id or session expired");
            AppError::unauthorized("Invalid session id or session expired")
        })?;

    if auth_session.expires_at.lt(&chrono::Utc::now().naive_utc()) {
        debug!("Refresh token is definitely expired");
        return Err(AppError::unauthorized("Refresh token expired"));
    }

    debug!("Finding user by id in session: {}", decoded_token.user_id);
    let user = UserRepository::find_user_by_id(connection, decoded_token.user_id)
        .await
        .ok_or_else(|| {
            debug!("Invalid user id");
            AppError::unauthorized("Referenced user does not exist")
        })?;

    debug!("Generating new access token");
    let access_token = helpers::token::generate_user_session_access_token(&user, &auth_session)?;

    debug("Login Complete with refresh token");
    Ok(AuthToken::new(access_token, refresh_token))
}
