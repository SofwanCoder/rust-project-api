use crate::contracts::user::{CreateUserPayload, UpdateUserPayload};
use crate::utilities::rand::generate_uuid;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Default, Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset, Identifiable,
)]
#[diesel(table_name = crate::schema::users)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, Default)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}
impl From<CreateUserPayload> for CreateUserModel {
    fn from(payload: CreateUserPayload) -> Self {
        CreateUserModel {
            id: generate_uuid(),
            name: payload.name,
            email: payload.email,
            password: crate::helpers::password::hash(payload.password).unwrap_or("".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable, AsChangeset, Default)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUserModel {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
impl From<UpdateUserPayload> for UpdateUserModel {
    fn from(payload: UpdateUserPayload) -> Self {
        UpdateUserModel {
            name: payload.name,
            email: payload.email,
            password: None,
        }
    }
}
