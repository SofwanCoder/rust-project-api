use crate::types::auths::AuthToken;
use crate::types::user::UserWithAuthInfo;
use crate::utilities::validation::email::unique_email_validator;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateUserPayload {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
    #[validate(
        email(message = "Email format is invalid"),
        custom(
            function = "unique_email_validator",
            arg = "&'v_a crate::database::ApplicationDatabase",
        )
    )]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be greater than 6 chars"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub authentication: AuthToken,
}

impl From<&UserWithAuthInfo> for CreateUserResponse {
    fn from(u: &UserWithAuthInfo) -> Self {
        CreateUserResponse {
            id: u.user.id,
            name: u.user.name.clone(),
            email: u.user.email.clone(),
            authentication: u.authentication.clone(),
        }
    }
}
