use actix_web::{http::StatusCode, HttpResponse};
use common::configs::constant;
use serde::{ser::SerializeStruct, Serialize, Serializer};
#[derive(Debug, Clone)]
pub enum AppResponse<'a, T> {
    Data(&'a str, Option<T>),
    Error(&'a str, Option<T>),
}

impl<'a, T> Serialize for AppResponse<'a, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            AppResponse::Data(ref message, ref data) => {
                let mut state = serializer.serialize_struct("Data", 2)?;
                state.serialize_field("message", *message)?;
                if data.is_some() {
                    state.serialize_field("data", data.as_ref().unwrap())?;
                }
                state.end()
            }
            AppResponse::Error(ref message, ref errors) => {
                let mut state = serializer.serialize_struct("Error", 2)?;
                state.serialize_field("message", message)?;
                if errors.is_some() {
                    state.serialize_field("errors", errors.as_ref().unwrap())?;
                }
                state.end()
            }
        }
    }
}

impl<'a, T> AppResponse<'a, T> {
    pub fn data(message: &'a str, data: Option<T>) -> AppResponse<'a, T> {
        AppResponse::Data(message, data)
    }

    pub fn error(message: &'a str, errors: Option<T>) -> AppResponse<'a, T> {
        AppResponse::Error(message, errors)
    }

    pub fn to_http_response(&self, status: StatusCode) -> HttpResponse
    where
        T: Serialize,
    {
        HttpResponse::build(status)
            .insert_header((constant::APP_RESPONSE_HEADER, "done"))
            .json(&self)
    }
}

pub fn ok<T>(data: T) -> HttpResponse
where
    T: Serialize,
{
    AppResponse::Data("success", Some(data)).to_http_response(StatusCode::OK)
}
