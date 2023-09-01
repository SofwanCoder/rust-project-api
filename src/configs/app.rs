use crate::helpers::response::AppResponse;
use actix_web::dev;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::ErrorHandlerResponse;

pub fn error_404_handler<B, E>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, E> {
    let app_response = AppResponse::<()> {
        message: "Resource not found".to_string(),
        data: None,
        errors: None,
    };

    let app_response_json = serde_json::to_string(&app_response).unwrap();
    let (req, res) = res.into_parts();
    let mut res = res.set_body(app_response_json.into_bytes().to_owned());

    res.headers_mut().insert(
        actix_web::http::header::HeaderName::from_static("content-type"),
        actix_web::http::header::HeaderValue::from_static("application/json"),
    );

    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}

pub fn error_default_handler<B, E>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>, E> {
    let app_response = AppResponse::<()> {
        message: "Unknown Error occurred".to_string(),
        data: None,
        errors: None,
    };

    let app_response_json = serde_json::to_string(&app_response).unwrap();
    let (req, res) = res.into_parts();
    let mut res = res.set_body(app_response_json.into_bytes().to_owned());

    res.headers_mut().insert(
        actix_web::http::header::HeaderName::from_static("content-type"),
        actix_web::http::header::HeaderValue::from_static("application/json"),
    );

    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}
