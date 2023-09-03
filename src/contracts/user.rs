use crate::models::user::UserModel;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateUser {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be greater than 6 chars"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct AuthenticatedUserResponse {
    pub token: String,
    pub user: UserModel,
}
