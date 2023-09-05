use crate::contracts::auth::CreateTokenPayload;
use crate::helpers::error::AppError;
use crate::helpers::response;
use crate::utilities::error::map_blocking_err_to_app_err;
use crate::utilities::error::map_validation_err_to_app_err;
use actix_web::{web, HttpRequest, Responder, Result};
use validator::Validate;

pub async fn create_tokens(
    req: HttpRequest,
    body: web::Json<CreateTokenPayload>,
) -> Result<impl Responder, AppError> {
    let db = req
        .app_data::<crate::database::ApplicationDatabase>()
        .unwrap()
        .clone();

    body.validate().map_err(map_validation_err_to_app_err)?;

    let result = web::block(move || {
        futures::executor::block_on(crate::services::auths::login(&db, body.into_inner()))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response::ok)
}
