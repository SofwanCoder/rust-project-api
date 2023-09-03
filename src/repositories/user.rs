use crate::database::DBConnection;
use crate::helpers::error::AppError;
use crate::models;
use crate::models::user::{CreateUserModel, UserModel};
use crate::schema::users::dsl::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub struct UserRepository {
    connection: DBConnection,
}

impl UserRepository {
    pub fn new(connection: DBConnection) -> Self {
        UserRepository { connection }
    }

    pub fn get_users(&mut self) -> Vec<models::user::UserModel> {
        let results = users
            .limit(5)
            .load::<models::user::UserModel>(&mut self.connection)
            .expect("Error loading users");
        results
    }

    pub fn find_user_by_id(&mut self, user_id: Uuid) -> Result<Option<UserModel>, AppError> {
        let results = users
            .find(user_id)
            .load::<models::user::UserModel>(&mut self.connection)
            .map_err(|_err| AppError::database_error("Error looking up user".to_string()))
            .map(|user_result| {
                if user_result.is_empty() {
                    None
                } else {
                    Some(user_result.first().unwrap().clone())
                }
            })?;
        Ok(results)
    }

    pub fn create_user(
        &mut self,
        data: CreateUserModel,
    ) -> Result<models::user::UserModel, AppError> {
        let result = diesel::insert_into(users)
            .values(&data)
            .get_results::<models::user::UserModel>(&mut self.connection)
            .map_err(|_err| AppError::database_error("Error creating user".to_string()))
            .map(|user_result| {
                if user_result.is_empty() {
                    None
                } else {
                    Some(user_result.first().unwrap().clone())
                }
            })?;
        if result.is_none() {
            return Err(AppError::database_error(
                "Unable to create user's account".to_string(),
            ));
        }
        return Ok(result.unwrap());
    }

    pub fn find_by_email(
        &mut self,
        user_email: String,
    ) -> Result<Option<models::user::UserModel>, AppError> {
        let user_optional = users
            .filter(email.eq(user_email))
            .load::<models::user::UserModel>(&mut self.connection)
            .map_err(|_err| AppError::database_error("Error finding user".to_string()))
            .map(|user_result| {
                if user_result.is_empty() {
                    None
                } else {
                    Some(user_result.first().unwrap().clone())
                }
            })?;
        return Ok(user_optional);
    }
}
