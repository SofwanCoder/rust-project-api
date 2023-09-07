use crate::database::DBConnection;
use crate::models;
use crate::models::user::{CreateUserModel, UserModel};
use crate::repositories::Repository;
use crate::schema::users::dsl::*;
use crate::utilities::error::map_diesel_err_to_app_err;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub struct UserRepository;

impl Repository for UserRepository {}

impl UserRepository {
    pub fn find_user_by_id(connection: DBConnection, user_id: Uuid) -> Option<UserModel> {
        users
            .find(user_id)
            .first::<models::user::UserModel>(connection)
            .optional()
            .map_err(map_diesel_err_to_app_err)
            .expect("Database error")
    }

    pub fn find_by_email(
        connection: DBConnection,
        user_email: String,
    ) -> Option<models::user::UserModel> {
        users
            .filter(email.eq(user_email))
            .first::<models::user::UserModel>(connection)
            .optional()
            .map_err(map_diesel_err_to_app_err)
            .expect("Database error")
    }

    pub fn create_user(connection: DBConnection, data: CreateUserModel) -> models::user::UserModel {
        diesel::insert_into(users)
            .values(&data)
            .get_result::<models::user::UserModel>(connection)
            .map_err(map_diesel_err_to_app_err)
            .expect("Database error")
    }
}
