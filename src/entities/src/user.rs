use async_trait::async_trait;
use helpers::password::hash_password;
use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument, trace};
use utilities::rand::generate_uuid;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing, skip_deserializing)]
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
    #[instrument(skip_all, fields(model = "users::Model"))]
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            trace!("New user, hashing password and generating uuid");
            self.password = Set(hash_password(self.password.unwrap()).unwrap());
            self.id = Set(generate_uuid());
        } else {
            trace!("Existing user, checking if password has changed");
            if self.password.is_set() {
                trace!("Password has changed, hashing new password");
                self.password = Set(hash_password(self.password.unwrap()).unwrap());
            }
        }
        debug!("Saved user: {:?}", self);
        Ok(self)
    }
}
