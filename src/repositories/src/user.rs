use crate::{
    contracts::{CreateUserRepositoryContract, UpdateUserRepositoryContract},
    Repository,
};
use common::{database::DBConnection, error::AppError};
pub use entities::user::{Entity as UserEntity, Model as UserModel};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use tracing::instrument;
use uuid::Uuid;

pub struct UserRepository;

impl Repository for UserRepository {}

impl UserRepository {
    #[instrument(skip(connection))]
    pub async fn find_users<C: DBConnection>(connection: &C) -> Vec<UserModel> {
        UserEntity::find()
            .all(connection)
            .await
            .map_err(AppError::database_error)
            .expect("Database error")
    }

    #[instrument(skip(connection))]
    pub async fn find_user_by_id<C: DBConnection>(
        connection: &C,
        user_id: Uuid,
    ) -> Option<UserModel> {
        UserEntity::find_by_id(user_id)
            .one(connection)
            .await
            .map_err(AppError::database_error)
            .expect("Database error")
    }

    #[instrument(skip(connection))]
    pub async fn find_by_email<C: DBConnection>(
        connection: &C,
        user_email: String,
    ) -> Option<UserModel> {
        UserEntity::find()
            .filter(entities::user::Column::Email.eq(user_email))
            .one(connection)
            .await
            .map_err(AppError::database_error)
            .expect("Database error")
    }

    #[instrument(skip(connection))]
    pub async fn create_user<C: DBConnection>(
        connection: &C,
        data: CreateUserRepositoryContract,
    ) -> Result<UserModel, AppError> {
        let user = entities::user::ActiveModel {
            name: Set(data.name),
            email: Set(data.email),
            password: Set(data.password),
            ..Default::default()
        };
        user.insert(connection)
            .await
            .map_err(AppError::database_error)
    }

    #[instrument(skip(connection))]
    pub async fn update_user<C: DBConnection>(
        connection: &C,
        user_id: Uuid,
        data: UpdateUserRepositoryContract,
    ) -> Result<UserModel, AppError> {
        let user = entities::user::ActiveModel {
            id: Set(user_id),
            name: data.name.map(Set).unwrap_or_default(),
            password: data.password.map(Set).unwrap_or_default(),
            email: data.email.map(Set).unwrap_or_default(),
            ..Default::default()
        };
        user.update(connection)
            .await
            .map_err(AppError::database_error)
    }
}
