use crate::helpers::response::AppResponse;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Display, Debug, Serialize)]
pub enum AppErrorKind {
    ValidationError,
    UserError,
    InternalError,
    AuthMissing,
    AuthDenied,
}

#[derive(Display, Debug)]
#[display(fmt = "{} {} {:?}", message, kind, data)]
pub struct AppError {
    #[display(fmt = "{}", message)]
    pub message: String,
    #[display(fmt = "{}", kind)]
    pub kind: AppErrorKind,
    #[display(fmt = "{:?}", data)]
    pub data: Option<HashMap<&'static str, String>>,
}

impl AppError {
    pub fn new(message: String, kind: AppErrorKind) -> AppError {
        AppError {
            message,
            kind,
            data: None,
        }
    }
    pub fn validation_error(
        message: String,
        data: Option<HashMap<&'static str, String>>,
    ) -> AppError {
        AppError {
            message,
            kind: AppErrorKind::ValidationError,
            data,
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.kind {
            AppErrorKind::ValidationError => StatusCode::EXPECTATION_FAILED,
            AppErrorKind::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorKind::UserError => StatusCode::BAD_REQUEST,
            AppErrorKind::AuthMissing => StatusCode::UNAUTHORIZED,
            AppErrorKind::AuthDenied => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        crate::helpers::response::app_http_response(
            self.status_code(),
            AppResponse::<HashMap<&'static str, String>> {
                message: self.message.clone(),
                data: self.data.clone(),
                errors: None,
            },
        )
    }
}
