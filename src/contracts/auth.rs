use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateTokenPayload {
    #[validate(email(message = "Email format is invalid"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be greater than 6 chars"))]
    pub password: String,
}
