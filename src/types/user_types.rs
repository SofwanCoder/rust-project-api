use crate::contracts::user_contract::CreateUserPayload;
use derive_more::Display;
use serde::Serialize;

#[derive(Display, Debug, Clone, Serialize)]
#[display(fmt = "{} {} {}", email, name, password)]
pub struct CreateUser {
    #[display(fmt = "{}", email)]
    pub email: String,
    #[display(fmt = "{}", name)]
    pub name: String,
    #[display(fmt = "***")]
    pub password: String,
}

impl From<CreateUserPayload> for CreateUser {
    fn from(payload: CreateUserPayload) -> Self {
        CreateUser {
            email: payload.email.clone(),
            name: payload.name.clone(),
            password: payload.password.clone(),
        }
    }
}
