use crate::{
    contracts::auth_contract::CreateTokenPayload,
    types::AuthenticatedData,
    utilities::error::map_validation_err_to_app_err,
    ApiResult,
};
use actix_web::{web, HttpMessage, HttpRequest, Responder, ResponseError};
use common::helpers::response;
use services;
use std::ops::Deref;
use tracing::instrument;
use validator::ValidateArgs;

#[instrument(skip_all)]
pub async fn create_token_controller(
    req: HttpRequest,
    body: web::Json<CreateTokenPayload>,
) -> ApiResult<impl Responder, impl ResponseError> {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    body.validate_args(body.deref())
        .map_err(map_validation_err_to_app_err)?;

    let result = services::auths::login_a_user(&ctx.db, body.into_inner()).await;

    result.map(response::ok)
}

#[instrument(skip_all)]
pub async fn delete_token_controller(req: HttpRequest) -> ApiResult {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    let authenticated_user = req.extensions().get::<AuthenticatedData>().unwrap().clone();

    let result = services::auths::logout_a_user(&ctx.db, authenticated_user).await;

    result.map(response::ok)
}
