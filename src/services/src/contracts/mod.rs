use derive_more::{DebugCustom, Display};
use serde::Serialize;

#[derive(Debug, Clone, Display, PartialEq)]
pub enum GrantType {
    Password,
    RefreshToken,
    AuthorizationCode,
}

#[derive(Debug, Clone)]
pub struct LoginUserContract {
    pub email: Option<String>,
    pub password: Option<String>,
    pub refresh_token: Option<String>,
    pub grant_type: GrantType,
}

#[derive(Debug, Clone)]
pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct UpdateUserPayloadContract {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdatePasswordContract {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Default, DebugCustom, Clone, Serialize)]
#[debug(fmt = "{} {} {}", email, name, password)]
pub struct CreateUserContract {
    pub email: String,
    pub name: String,
    pub password: String,
}
