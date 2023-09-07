use crate::database::DBConnection;
use crate::models;
use crate::models::auth::{AuthModel, CreateAuthModel};
use crate::repositories::Repository;
use crate::schema::auths::dsl::*;
use crate::utilities::error::map_diesel_err_to_app_err;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub struct AuthRepository;

impl Repository for AuthRepository {}

impl AuthRepository {
    pub fn find_auth_by_id(connection: DBConnection, auth_id: Uuid) -> Option<AuthModel> {
        auths
            .find(auth_id)
            .first::<models::auth::AuthModel>(connection)
            .optional()
            .map_err(map_diesel_err_to_app_err)
            .expect("Database error")
    }

    pub fn create_auth(connection: DBConnection, auth_data: CreateAuthModel) -> AuthModel {
        diesel::insert_into(auths)
            .values(&auth_data)
            .get_result::<models::auth::AuthModel>(connection)
            .map_err(map_diesel_err_to_app_err)
            .expect("Database error")
    }

    pub fn delete_auth_by_id(connection: DBConnection, auth_id: Uuid) -> () {
        diesel::delete(auths)
            .filter(id.eq(auth_id))
            .execute(connection)
            .map_err(map_diesel_err_to_app_err)
            .expect("Database error");
    }
}
