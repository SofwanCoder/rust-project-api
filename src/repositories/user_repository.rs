use crate::{
    contracts::user::UpdateUserPayload,
    database::DBConnection,
    helpers::error::AppError,
    models,
    models::user::{Entity as UserEntity, Model as UserModel},
    repositories::Repository,
    types::user_types::CreateUser,
};
use futures_util::TryFutureExt;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub struct UserRepository;

impl Repository for UserRepository {}

impl UserRepository {
    pub async fn find_users<C: DBConnection>(connection: &C) -> Vec<UserModel> {
        UserEntity::find()
            .all(connection)
            .map_err(|_| AppError::database_error("Database error"))
            .await
            .expect("Database error")
    }

    pub async fn find_user_by_id<C: DBConnection>(
        connection: &C,
        user_id: Uuid,
    ) -> Option<UserModel> {
        UserEntity::find_by_id(user_id)
            .one(connection)
            .map_err(|_| AppError::database_error("Database error"))
            .await
            .expect("Database error")
    }

    pub async fn find_by_email<C: DBConnection>(
        connection: &C,
        user_email: String,
    ) -> Option<UserModel> {
        UserEntity::find()
            .filter(models::user::Column::Email.eq(user_email))
            .one(connection)
            .map_err(|_| AppError::database_error("Database error"))
            .await
            .expect("Database error")
    }

    pub async fn create_user<C: DBConnection>(
        connection: &C,
        data: CreateUser,
    ) -> Result<UserModel, AppError> {
        let user = models::user::ActiveModel {
            name: Set(data.name),
            email: Set(data.email),
            password: Set(data.password),
            ..Default::default()
        };
        user.insert(connection)
            .await
            .map_err(|e| AppError::database_error(e))
    }

    pub async fn update_user<C: DBConnection>(
        connection: &C,
        user_id: Uuid,
        data: UpdateUserPayload,
    ) -> UserModel {
        let user = models::user::ActiveModel {
            id: Set(user_id),
            name: if data.name.is_some() {
                Set(data.name.unwrap())
            } else {
                Default::default()
            },
            ..Default::default()
        };
        user.update(connection).await.expect("Database error")
    }
}
