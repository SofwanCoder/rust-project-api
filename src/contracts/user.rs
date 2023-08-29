use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
    #[validate(
        email,
        contains(pattern = "gmail", message = "Email must be valid gmail address")
    )]
    pub email: String,
    pub password: String,
}
