use crate::models::user::UserModel;
use crate::types::auths::AuthToken;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserWithAuthInfo {
    pub authentication: AuthToken,
    pub user: UserModel,
}
