use crate::contracts::user::{CreateUserPayload, UpdatePasswordPayload, UpdateUserPayload};
use crate::helpers::error::AppError;
use crate::helpers::response;
use crate::types::auths::AuthenticatedData;
use crate::utilities::error::{map_blocking_err_to_app_err, map_validation_err_to_app_err};
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use uuid::Uuid;
use validator::{Validate, ValidateArgs};

pub async fn create_user(
    req: HttpRequest,
    body: web::Json<CreateUserPayload>,
) -> Result<impl Responder, AppError> {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    let result = web::block(move || {
        body.validate_args(&ctx.db)
            .map_err(map_validation_err_to_app_err)?;

        futures::executor::block_on(crate::services::users::register(
            &ctx,
            body.into_inner().into(),
        ))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response::ok)
}

pub async fn fetch_user(
    req: HttpRequest,
    user_id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    let user_id = user_id.into_inner();

    let result = web::block(move || {
        futures::executor::block_on(crate::services::users::fetch_user(&ctx.db, user_id))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response::ok)
}

pub async fn fetch_me(req: HttpRequest) -> Result<impl Responder, AppError> {
    let user_id = req.extensions().get::<AuthenticatedData>().unwrap().user_id;

    let web_path_user_id = web::Path::try_from(user_id.clone());

    if web_path_user_id.is_err() {
        return Err(AppError::internal_server(web_path_user_id.err().unwrap()));
    }

    let web_path_user_id = web_path_user_id.unwrap();

    fetch_user(req, web_path_user_id).await
}

pub async fn fetch_users(req: HttpRequest) -> Result<impl Responder, AppError> {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    let result = web::block(move || {
        futures::executor::block_on(crate::services::users::fetch_users(&ctx.db))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response::ok)
}

pub async fn update_user(
    req: HttpRequest,
    user_id: web::Path<Uuid>,
    body: web::Json<UpdateUserPayload>,
) -> Result<impl Responder, AppError> {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();
    let user_id = user_id.into_inner();

    let result = web::block(move || {
        body.validate_args(&ctx.db)
            .map_err(map_validation_err_to_app_err)?;

        futures::executor::block_on(crate::services::users::update_user(
            &ctx.db,
            user_id,
            body.into_inner().into(),
        ))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response::ok)
}

pub async fn update_password(
    req: HttpRequest,
    user_id: web::Path<Uuid>,
    body: web::Json<UpdatePasswordPayload>,
) -> Result<impl Responder, AppError> {
    body.validate().map_err(map_validation_err_to_app_err)?;

    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    let user_id = user_id.into_inner();

    let result = web::block(move || {
        futures::executor::block_on(crate::services::users::update_password(
            &ctx.db,
            user_id,
            body.into_inner().into(),
        ))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response::ok)
}
