use crate::helpers::response;
use crate::helpers::response::AppResponse;
use actix_web::error::JsonPayloadError;
use actix_web::http::StatusCode;
use actix_web::{web, Error};
use std::ops::Add;

fn error_handler(err: JsonPayloadError, _req: &actix_web::HttpRequest) -> Error {
    let response = match &err {
        JsonPayloadError::Deserialize(json_err) => {
            let mut message = "JSON::".to_string();
            if json_err.is_syntax() {
                message = message.add("Syntax Error: ");
            }
            if json_err.is_data() {
                message = message.add("Data Error: ");
            } else if json_err.is_eof() {
                message = message.add("EOF Error: ");
            } else if json_err.is_io() {
                message = message.add("IO Error: ");
            } else {
                message = message.add("Unknown Error: ");
            }

            response::app_http_response(
                StatusCode::BAD_REQUEST,
                AppResponse::<()> {
                    message: message.add(json_err.to_string().as_str()),
                    errors: None,
                    data: None,
                },
            )
        }
        _ => response::app_http_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            AppResponse::<()> {
                message: "JSON: Unknown Error".to_string(),
                errors: None,
                data: None,
            },
        ),
    };
    actix_web::error::InternalError::from_response(err, response).into()
}

pub fn get_json_config() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(4096)
        .error_handler(error_handler)
}
