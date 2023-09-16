use crate::helpers::password::hash;
use crate::utilities::rand::generate_uuid;
use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing, skip_deserializing, default = "String::new")]
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::auth::Entity")]
    Auth,
}

impl Related<super::auth::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Auth.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.password = Set(hash(self.password.unwrap()).unwrap());
            self.id = Set(generate_uuid());
        }
        Ok(self)
    }
}
