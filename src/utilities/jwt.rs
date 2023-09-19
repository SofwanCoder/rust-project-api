use crate::helpers::error_helper::AppError;
use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, error, instrument};

#[instrument(skip_all)]
pub fn encode<D>(data: D) -> Result<String, AppError>
where
    D: Serialize,
{
    debug!("Attempting to encode data");
    let secret_key = crate::configs::settings::Variables::jwt_secret_key();
    let secret_key = secret_key.as_bytes();
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &data,
        &jsonwebtoken::EncodingKey::from_secret(secret_key),
    )
    .map_err(|e| AppError::internal_server(e))?;
    debug!("Encoded data successfully");
    return Ok(token);
}

#[instrument(skip_all)]
pub fn decode<D>(token: &str) -> Result<D, AppError>
where
    D: DeserializeOwned,
{
    debug!("Attempting to decode data");
    let secret_key = crate::configs::settings::Variables::jwt_secret_key();
    let secret_key = secret_key.as_bytes();
    let token = jsonwebtoken::decode::<D>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret_key),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|e| {
        error!("Error decoding token: {:?}", e);
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::expired_data("Token expired")
            }
            _ => AppError::internal_server(e),
        }
    })?;
    debug!("Decoded data successful");
    return Ok(token.claims);
}
