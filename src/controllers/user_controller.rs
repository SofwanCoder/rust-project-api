use crate::{
    contracts::user_contract::{CreateUserPayload, UpdatePasswordPayload, UpdateUserPayload},
    helpers::{error_helper::AppError, response_helper},
    types::auth_types::AuthenticatedData,
    utilities::error::{map_blocking_err_to_app_err, map_validation_err_to_app_err},
};
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use uuid::Uuid;
use validator::{Validate, ValidateArgs};

pub async fn create_user_controller(
    req: HttpRequest,
    body: web::Json<CreateUserPayload>,
) -> Result<impl Responder, AppError> {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    let result = web::block(move || {
        body.validate_args(&ctx.db)
            .map_err(map_validation_err_to_app_err)?;

        futures::executor::block_on(crate::services::user_service::register(
            &ctx,
            body.into_inner().into(),
        ))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response_helper::ok)
}

pub async fn fetch_user_controller(
    req: HttpRequest,
    user_id: web::Path<Uuid>,
) -> Result<impl Responder, AppError> {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    let user_id = user_id.into_inner();

    let result = crate::services::user_service::fetch_user(&ctx, user_id).await;

    result.map(response_helper::ok)
}

pub async fn fetch_me_controller(req: HttpRequest) -> Result<impl Responder, AppError> {
    let user_id = req.extensions().get::<AuthenticatedData>().unwrap().user_id;

    let web_path_user_id = web::Path::try_from(user_id.clone());

    if web_path_user_id.is_err() {
        return Err(AppError::internal_server(web_path_user_id.err().unwrap()));
    }

    let web_path_user_id = web_path_user_id.unwrap();

    fetch_user_controller(req, web_path_user_id).await
}

pub async fn fetch_users_controller(req: HttpRequest) -> Result<impl Responder, AppError> {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    let result = crate::services::user_service::fetch_users(&ctx.db).await;

    result.map(response_helper::ok)
}

pub async fn update_user_controller(
    req: HttpRequest,
    user_id: web::Path<Uuid>,
    body: web::Json<UpdateUserPayload>,
) -> Result<impl Responder, AppError> {
    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();
    let user_id = user_id.into_inner();

    let result = web::block(move || {
        body.validate_args(&ctx.db)
            .map_err(map_validation_err_to_app_err)?;

        futures::executor::block_on(crate::services::user_service::update_user(
            &ctx.db,
            user_id,
            body.into_inner().into(),
        ))
    })
    .await
    .map_err(map_blocking_err_to_app_err)?;

    result.map(response_helper::ok)
}

pub async fn update_password_controller(
    req: HttpRequest,
    user_id: web::Path<Uuid>,
    body: web::Json<UpdatePasswordPayload>,
) -> Result<impl Responder, AppError> {
    body.validate().map_err(map_validation_err_to_app_err)?;

    let ctx = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    let user_id = user_id.into_inner();

    let result =
        crate::services::user_service::update_password(&ctx.db, user_id, body.into_inner().into())
            .await;

    result.map(response_helper::ok)
}
