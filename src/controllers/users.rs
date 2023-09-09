use crate::contracts::user::CreateUserPayload;
use crate::helpers::error::AppError;
use crate::helpers::response;
use crate::utilities::error::map_blocking_err_to_app_err;
use crate::utilities::error::map_validation_err_to_app_err;
use actix_web::{web, HttpRequest, Responder, Result};
use uuid::Uuid;
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

pub async fn fetch_user(
    req: HttpRequest,
    user_id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let db = req
        .app_data::<crate::database::ApplicationDatabase>()
        .unwrap()
        .clone();

    let user_id = user_id.into_inner();

    let result = web::block(move || {
        futures::executor::block_on(crate::services::users::fetch_user(&db, user_id))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response::ok)
}
