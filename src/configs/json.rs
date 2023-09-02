use crate::helpers::response;
use crate::helpers::response::AppResponse;
use actix_web::error::JsonPayloadError;
use actix_web::http::StatusCode;
use actix_web::{web, Error};

const JSON_PAYLOAD_LIMIT: usize = 4096;

fn error_handler(err: JsonPayloadError, _req: &actix_web::HttpRequest) -> Error {
    let response = match &err {
        JsonPayloadError::Deserialize(json_err) => response::app_http_response(
            StatusCode::BAD_REQUEST,
            AppResponse::<()> {
                message: json_err.to_string(),
                errors: None,
                data: None,
            },
        ),
        _ => response::app_http_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            AppResponse::<()> {
                message: "Unknown Error".to_string(),
                errors: None,
                data: None,
            },
        ),
    };
    actix_web::error::InternalError::from_response(err, response).into()
}

pub fn get_json_config() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(JSON_PAYLOAD_LIMIT)
        .error_handler(error_handler)
}
