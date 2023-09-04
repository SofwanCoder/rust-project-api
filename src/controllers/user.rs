use crate::contracts::user::CreateUserPayload;
use crate::helpers::error::AppError;
use crate::helpers::response;
use crate::utilities::validation::map_to_validation_err;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use std::ops::Deref;
use validator::ValidateArgs;

pub async fn create(
    req: HttpRequest,
    body: web::Json<CreateUserPayload>,
) -> Result<impl Responder, AppError> {
    let db = req
        .app_data::<crate::database::ApplicationDatabase>()
        .unwrap();

    body.validate_args(db).map_err(map_to_validation_err)?;

    let result = crate::services::user::register(db, body.into_inner()).await;

    result.map(|user| response::ok(user))
}

pub async fn fetch(req: HttpRequest) -> Result<impl Responder, AppError> {
    let extensions = req.extensions();

    let authenticated_user = extensions
        .deref()
        .get::<crate::types::auths::AuthenticatedData>()
        .unwrap()
        .clone();

    let db = req
        .app_data::<crate::database::ApplicationDatabase>()
        .unwrap()
        .clone();

    let result =
        web::block(move || crate::services::user::fetch(&db, authenticated_user.user_id.clone()))
            .await
            .map_err(|err| {
                log::error!("Error: {:?}", err);
                AppError::internal_server("Error handling request".to_string())
            })?;

    result.map(|user| response::ok(user))
}
