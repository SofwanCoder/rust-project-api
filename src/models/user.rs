use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UserModel {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct CreateUserModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}
