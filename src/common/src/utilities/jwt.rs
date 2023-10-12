use crate::error::AppError;
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

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Dummy, Fake, Faker};
    use serde::Deserialize;

    #[derive(Debug, Serialize, Deserialize, Dummy, PartialEq)]
    pub struct DataStruct {
        val: String,
        iat: usize,
        exp: usize,
    }

    #[test]
    fn test_encode_decode() {
        let data_struct = DataStruct {
            iat: chrono::Utc::now().timestamp() as usize,
            exp: (chrono::Utc::now().timestamp() + 10000) as usize,
            ..Faker.fake()
        };
        let token = encode(&data_struct).unwrap();
        assert_eq!(token.split(".").count(), 3);
        let decoded: DataStruct = decode(&token).unwrap();
        assert_eq!(data_struct, decoded);
    }
}
