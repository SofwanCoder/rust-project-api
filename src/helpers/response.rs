use crate::configs::constant;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AppResponse<T> {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<T>,
}

pub fn app_http_response<T>(status: StatusCode, app_response: AppResponse<T>) -> HttpResponse
where
    T: Serialize,
{
    HttpResponse::build(status)
        .insert_header((constant::APP_RESPONSE_HEADER, "done"))
        .json(app_response)
}

pub fn ok<T>(data: T) -> HttpResponse
where
    T: Serialize,
{
    HttpResponse::Ok().json(AppResponse {
        message: "OK".to_string(),
        data: Some(data),
        errors: None,
    })
}
