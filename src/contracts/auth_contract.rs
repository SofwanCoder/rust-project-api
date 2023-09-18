use crate::helpers::validation::auth_validation::validate_grant_type;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default, PartialEq)]
pub enum GrantType {
    #[default]
    #[serde(rename = "password")]
    #[display(fmt = "password")]
    Password,
    #[serde(rename = "refresh_token")]
    #[display(fmt = "refresh_token")]
    RefreshToken,
    #[serde(rename = "authorization_code")]
    #[display(fmt = "authorization_code")]
    AuthorizationCode,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateTokenPayload {
    #[serde(default)]
    #[validate(email(message = "Email format is invalid"))]
    pub email: Option<String>,
    #[serde(default)]
    #[validate(length(min = 6, message = "Password must be greater than 6 chars"))]
    pub password: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    #[validate(custom(function = "validate_grant_type", arg = "&'v_a CreateTokenPayload"))]
    pub grant_type: GrantType,
}
