use crate::helpers::response;
use actix_web::dev;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::ErrorHandlerResponse;

pub fn error_404_handler<B, E>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, E> {
    let status = res.status();
    let request = res.into_parts().0;

    let new_response = response::app_http_response(
        status,
        response::AppResponse::<()> {
            message: "Resource not found".to_string(),
            data: None,
            errors: None,
        },
    );

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, new_response).map_into_right_body(),
    ))
}

pub fn error_default_handler<B, E>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>, E> {
    let status = res.status();
    let request = res.into_parts().0;

    let new_response = response::app_http_response(
        status,
        response::AppResponse::<()> {
            message: "Something went wrong".to_string(),
            data: None,
            errors: None,
        },
    );

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, new_response).map_into_right_body(),
    ))
}
