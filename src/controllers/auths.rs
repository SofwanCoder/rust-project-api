use crate::contracts::auth::CreateTokenPayload;
use crate::helpers::error::AppError;
use crate::helpers::response;
use crate::utilities::error::map_blocking_err_to_app_err;
use crate::utilities::error::map_validation_err_to_app_err;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use std::ops::Deref;
use validator::ValidateArgs;

pub async fn create_token(
    req: HttpRequest,
    body: web::Json<CreateTokenPayload>,
) -> Result<impl Responder, AppError> {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    body.validate_args(body.deref())
        .map_err(map_validation_err_to_app_err)?;

    let result = web::block(move || {
        futures::executor::block_on(crate::services::auths::login(&ctx.db, body.into_inner()))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response::ok)
}

pub async fn delete_token(req: HttpRequest) -> Result<impl Responder, AppError> {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    let authenticated_user = req
        .extensions()
        .get::<crate::types::auths::AuthenticatedData>()
        .unwrap()
        .clone();

    let result = web::block(move || {
        futures::executor::block_on(crate::services::auths::logout(&ctx.db, authenticated_user))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response::ok)
}
