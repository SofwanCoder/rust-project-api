#![allow(dead_code)]
use crate::helpers::response::AppResponse;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::Serialize;
use std::{collections::HashMap, fmt::Display};

#[derive(Display, Debug, Serialize, PartialEq)]
pub enum AppErrorKind {
    ValidationError,
    DatabaseError,
    ResourceNotFound,
    InternalError,
    AuthorizationError,
    AuthDenied,
    BadClientError,
    DataExpired,
}

#[derive(Display, Debug, Error)]
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
    pub fn new<T: Display>(message: T, kind: AppErrorKind) -> AppError {
        AppError {
            message: message.to_string(),
            kind,
            data: None,
        }
    }

    pub fn validation_error<T: Display>(
        message: T,
        data: Option<HashMap<&'static str, String>>,
    ) -> AppError {
        AppError {
            message: message.to_string(),
            kind: AppErrorKind::ValidationError,
            data,
        }
    }

    pub fn database_error<T: Display>(message: T) -> AppError {
        AppError {
            message: message.to_string(),
            kind: AppErrorKind::DatabaseError,
            data: None,
        }
    }

    pub fn connection_error<T: Display>(message: T) -> AppError {
        AppError {
            message: message.to_string(),
            kind: AppErrorKind::InternalError,
            data: None,
        }
    }

    pub fn internal_server<T: Display>(message: T) -> AppError {
        AppError {
            message: message.to_string(),
            kind: AppErrorKind::InternalError,
            data: None,
        }
    }

    pub fn unauthorized<T: Display>(message: T) -> AppError {
        AppError {
            message: message.to_string(),
            kind: AppErrorKind::AuthorizationError,
            data: None,
        }
    }

    pub fn forbidden<T: Display>(message: T) -> AppError {
        AppError {
            message: message.to_string(),
            kind: AppErrorKind::AuthDenied,
            data: None,
        }
    }

    pub fn client_error<T: Display>(message: T) -> AppError {
        AppError {
            message: message.to_string(),
            kind: AppErrorKind::BadClientError,
            data: None,
        }
    }

    pub fn not_found<T: Display>(message: T) -> AppError {
        AppError {
            message: message.to_string(),
            kind: AppErrorKind::BadClientError,
            data: None,
        }
    }

    pub fn expired_data<T: Display>(message: T) -> AppError {
        AppError {
            message: message.to_string(),
            kind: AppErrorKind::BadClientError,
            data: None,
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.kind {
            AppErrorKind::ValidationError => StatusCode::EXPECTATION_FAILED,
            AppErrorKind::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorKind::InternalError => StatusCode::NOT_IMPLEMENTED,
            AppErrorKind::AuthorizationError => StatusCode::UNAUTHORIZED,
            AppErrorKind::ResourceNotFound => StatusCode::NOT_FOUND,
            AppErrorKind::AuthDenied => StatusCode::FORBIDDEN,
            AppErrorKind::BadClientError => StatusCode::BAD_REQUEST,
            AppErrorKind::DataExpired => StatusCode::GONE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        AppResponse::Error(self.message.as_str(), self.data.as_ref())
            .to_http_response(self.status_code())
    }
}
