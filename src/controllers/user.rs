use crate::contracts::user::CreateUser;
use crate::helpers::error::AppError;
use crate::helpers::response;
use crate::utilities::validation::validate_request_data;
use actix_web::{web, HttpMessage, HttpRequest, Responder, Result};
use std::ops::Deref;

pub async fn create(
    req: HttpRequest,
    body: web::Json<CreateUser>,
) -> Result<impl Responder, AppError> {
    validate_request_data(body.deref())?;
    let extensions = req.extensions();

    let authenticated_user = extensions
        .deref()
        .get::<crate::types::auths::AuthenticatedData>();

    if authenticated_user.is_some() {
        println!(
            "authenticated_user: {:?}",
            authenticated_user.unwrap().user_id
        );
    }

    let result = crate::services::user::register(body.into_inner()).await;

    result.map(|user| response::ok(user))
}
