use crate::utilities::validation::auths::validate_grant_type;
use serde::{Deserialize, Serialize};
use validator::{Validate};

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
    #[validate(
        length(min = 1, message = "Grant type must be provided"),
        custom(function = "validate_grant_type", arg = "&'v_a CreateTokenPayload")
    )]
    pub grant_type: String,
}
