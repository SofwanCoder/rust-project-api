use crate::database::DBConnection;
use crate::helpers::error::{AppError, AppErrorKind};
use crate::models;
use crate::models::user::{CreateUserModel, UserModel};
use crate::schema::users::dsl::*;
use diesel::{QueryDsl, RunQueryDsl};

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

    pub fn find_user(&mut self, user_id: String) -> Result<Option<UserModel>, AppError> {
        let results = users
            .find(user_id)
            .load::<models::user::UserModel>(&mut self.connection)
            .map_err(|_err| {
                AppError::new(
                    "Error finding user".to_string(),
                    AppErrorKind::DatabaseError,
                )
            })
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
        let results = diesel::insert_into(users)
            .values(&data)
            .get_results::<models::user::UserModel>(&mut self.connection);

        if results.is_err() {
            log::error!("Error: {:?}", results.err().unwrap().to_string());
            return Err(AppError::new(
                "Error creating user".to_string(),
                AppErrorKind::DatabaseError,
            ));
        }

        let user = results.unwrap().first().unwrap().clone();
        return Ok(user);
    }
}
