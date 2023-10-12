use crate::{contracts::CreateAuthRepositoryContract, Repository};
use common::{database::DBConnection, error::AppError, rand::generate_uuid};
use entities::{
    self,
    auth::{Entity as AuthEntity, Model as AuthModel},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DeleteResult, EntityTrait};
use uuid::Uuid;

pub struct AuthRepository;

impl Repository for AuthRepository {}

impl AuthRepository {
    pub async fn find_auth_by_id<C: DBConnection>(
        connection: &C,
        auth_id: Uuid,
    ) -> Option<AuthModel> {
        AuthEntity::find_by_id(auth_id)
            .one(connection)
            .await
            .map_err(AppError::database_error)
            .expect("Database error")
    }

    pub async fn create_auth<C: DBConnection>(
        connection: &C,
        auth_data: CreateAuthRepositoryContract,
    ) -> Result<AuthModel, AppError> {
        let auth = entities::auth::ActiveModel {
            id: Set(generate_uuid()),
            user_id: Set(auth_data.user_id),
            expires_at: Set(auth_data.expires_at),
            ..Default::default()
        };
        auth.insert(connection)
            .await
            .map_err(AppError::database_error)
    }

    pub async fn delete_auth_by_id<C: DBConnection>(
        connection: &C,
        auth_id: Uuid,
    ) -> Result<u64, AppError> {
        let delete_result: DeleteResult = AuthEntity::delete_by_id(auth_id)
            .exec(connection)
            .await
            .map_err(AppError::database_error)?;
        Ok(delete_result.rows_affected)
    }
}
