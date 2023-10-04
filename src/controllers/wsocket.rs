use crate::{api::ApiResult, services::wsocket};
use actix_web::{web, HttpRequest};
use actix_web_actors::ws;
use common::error::AppError;

pub async fn index(req: HttpRequest, stream: web::Payload) -> ApiResult {
    ws::start(wsocket::AppWebSocket, &req, stream).map_err(|e| AppError::internal_server(e))
}
