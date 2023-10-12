use crate::ApiResult;
use actix_web::{web, HttpRequest};
use actix_web_actors::ws;
use common::error::AppError;
use services::wsocket;

pub async fn index(req: HttpRequest, stream: web::Payload) -> ApiResult {
    ws::start(wsocket::AppWebSocket, &req, stream).map_err(AppError::internal_server)
}
