use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn encode<D>(data: D) -> Result<String, jsonwebtoken::errors::Error>
where
    D: Serialize,
{
    let secret_key = crate::configs::settings::Variables::jwt_secret_key();
    let secret_key = secret_key.as_bytes();
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &data,
        &jsonwebtoken::EncodingKey::from_secret(secret_key),
    )?;
    return Ok(token);
}

pub fn decode<D>(token: &str) -> Result<D, jsonwebtoken::errors::Error>
where
    D: DeserializeOwned,
{
    let secret_key = crate::configs::settings::Variables::jwt_secret_key();
    let secret_key = secret_key.as_bytes();
    let token = jsonwebtoken::decode::<D>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret_key),
        &jsonwebtoken::Validation::default(),
    )?;
    return Ok(token.claims);
}
