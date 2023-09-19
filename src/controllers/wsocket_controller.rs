use crate::{helpers::error::AppError, services::wsocket};
use actix_web::{web, HttpRequest, Responder, Result};
use actix_web_actors::ws;

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<impl Responder, AppError> {
    ws::start(wsocket::AppWebSocket, &req, stream).map_err(|e| AppError::internal_server(e))
}
