use crate::contracts::user::CreateUserPayload;
use crate::helpers::error::AppError;
use crate::helpers::response;
use crate::utilities::error::map_blocking_err_to_app_err;
use crate::utilities::error::map_validation_err_to_app_err;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use std::ops::Deref;
use validator::ValidateArgs;

pub async fn create(
    req: HttpRequest,
    body: web::Json<CreateUserPayload>,
) -> Result<impl Responder, AppError> {
    let db = req
        .app_data::<crate::database::ApplicationDatabase>()
        .unwrap()
        .clone();

    let result = web::block(move || {
        body.validate_args(&db)
            .map_err(map_validation_err_to_app_err)?;

        futures::executor::block_on(crate::services::users::register(&db, body.into_inner()))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response::ok)
}

pub async fn fetch(req: HttpRequest) -> Result<impl Responder, AppError> {
    let authenticated_user = req
        .extensions()
        .get::<crate::types::auths::AuthenticatedData>()
        .unwrap()
        .clone();

    let db = req
        .app_data::<crate::database::ApplicationDatabase>()
        .unwrap()
        .clone();

    let result = web::block(move || {
        futures::executor::block_on(crate::services::users::fetch(
            &db,
            authenticated_user.user_id,
        ))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response::ok)
}
