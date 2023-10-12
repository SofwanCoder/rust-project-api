use crate::contracts::user_contract::{CreateUserPayload, UpdatePasswordPayload};
use derive_more::DebugCustom;
use serde::Serialize;



#[derive(Default, DebugCustom, Clone, Serialize)]
#[debug(
    fmt = "UpdateUser {{ email: {:?} name: {:?} password: *redacted* }}",
    email,
    name
)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Default, Clone, Serialize, DebugCustom)]
#[debug(fmt = "UpdatePassword {{ current_password: *redacted*, new_password: *redacted* }}")]
pub struct UpdatePassword {
    pub current_password: String,
    pub new_password: String,
}

impl From<CreateUserPayload> for CreateUser {
    fn from(payload: CreateUserPayload) -> Self {
        CreateUser {
            email: payload.email,
            name: payload.name,
            password: payload.password,
        }
    }
}

impl From<UpdatePasswordPayload> for UpdatePassword {
    fn from(payload: UpdatePasswordPayload) -> Self {
        UpdatePassword {
            current_password: payload.current_password,
            new_password: payload.new_password,
        }
    }
}
