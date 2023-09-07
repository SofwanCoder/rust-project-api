use crate::models::user::UserModel;
use crate::utilities::rand::generate_uuid;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::auths)]
pub struct AuthModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::auths)]
pub struct CreateAuthModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: chrono::NaiveDateTime,
}

impl From<&UserModel> for CreateAuthModel {
    fn from(user: &UserModel) -> Self {
        CreateAuthModel {
            id: generate_uuid(),
            user_id: user.id,
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::days(30),
        }
    }
}
