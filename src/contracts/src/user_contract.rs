use helpers::validation::email::unique_email_validator;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateUserPayload {
    #[serde(default)]
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
    #[serde(default)]
    #[validate(
        email(message = "Email format is invalid"),
        custom(
            function = "unique_email_validator",
            arg = "&'v_a crate::database::ApplicationDatabase",
        )
    )]
    pub email: String,
    #[serde(default)]
    #[validate(length(min = 6, message = "Password must be greater than 6 chars"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct UpdateUserPayload {
    #[serde(default)]
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: Option<String>,
    #[serde(default)]
    #[validate(
        email(message = "Email format is invalid"),
        custom(
            function = "unique_email_validator",
            arg = "&'v_a crate::database::ApplicationDatabase",
        )
    )]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct UpdatePasswordPayload {
    #[serde(default)]
    #[validate(length(min = 3, message = "Password must be greater than 3 chars"))]
    pub current_password: String,
    #[serde(default)]
    #[validate(length(min = 3, message = "New password must be greater than 3 chars"))]
    pub new_password: String,
}
