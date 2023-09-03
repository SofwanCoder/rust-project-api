use crate::contracts::user::CreateUserPayload;
use crate::helpers::error::AppError;
use crate::helpers::response;
use crate::utilities::validation::validate_request_with_database;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use std::ops::Deref;

pub async fn create(
    req: HttpRequest,
    body: web::Json<CreateUserPayload>,
) -> Result<impl Responder, AppError> {
    let db = req
        .app_data::<crate::database::ApplicationDatabase>()
        .unwrap();

    validate_request_with_database(body.deref().clone(), db)?;

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
        .unwrap();

    let result = crate::services::user::fetch(db, authenticated_user.user_id).await;

    result.map(|user| response::ok(user))
}
