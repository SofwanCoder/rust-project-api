use crate::helpers::error::AppError;
use crate::models::auth::AuthModel;
use crate::models::user::UserModel;
use crate::types::auths::{AuthenticatedData, RefreshTokenData};
use crate::utilities;

pub fn generate_user_session_access_token(
    user: &UserModel,
    auth_session: &AuthModel,
) -> Result<String, AppError> {
    let expires_in_24_hours = chrono::Utc::now().naive_utc() + chrono::Duration::days(1);
    utilities::jwt::encode(AuthenticatedData {
        session_id: auth_session.id,
        user_id: user.id,
        clearance_level: 1,
        iat: chrono::Utc::now().timestamp() as usize,
        exp: expires_in_24_hours.timestamp() as usize,
    })
    .map_err(|_| AppError::internal_server("Error requesting access token".to_string()))
}

pub fn generate_user_session_refresh_token(auth_session: &AuthModel) -> Result<String, AppError> {
    utilities::jwt::encode(RefreshTokenData::from(auth_session))
        .map_err(|_| AppError::internal_server("Error requesting refresh token".to_string()))
}

pub fn decode_token_data_for_session(token: &String) -> Result<RefreshTokenData, AppError> {
    utilities::jwt::decode::<RefreshTokenData>(token).map_err(|e| {
        let error_kind = e.kind();
        let message = match error_kind {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Refresh token expired",
            _ => "Invalid refresh token",
        };
        AppError::unauthorized(message.to_string())
    })
}
