use crate::contracts::user::CreateUser;
use crate::helpers::error::AppError;
use crate::helpers::response;
use crate::utilities::validation::validate_request_data;
use actix_web::{web, HttpRequest, Responder, Result};
use std::ops::Deref;

pub async fn create(body: web::Json<CreateUser>) -> Result<impl Responder, AppError> {
    validate_request_data(body.deref())?;

    println!("user::create");
    let result = crate::services::user::register(body.into_inner()).await;

    result.map(|user| response::ok(user))
}
