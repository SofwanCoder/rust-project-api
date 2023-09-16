use crate::{configs::constant, helpers::response_helper};
use actix_web::{dev, dev::ServiceResponse, middleware::ErrorHandlerResponse};

pub fn error_404_handler<B, E>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, E> {
    let status = res.status();
    let request = res.into_parts().0;

    let new_response = response_helper::app_http_response(
        status,
        response_helper::AppResponse::<()> {
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
    let (request, response) = res.into_parts();

    let has_already_responded = response
        .headers()
        .contains_key(constant::APP_RESPONSE_HEADER);

    if has_already_responded {
        return Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
            request,
            response.map_into_left_body(),
        )));
    }

    let new_response = response_helper::app_http_response(
        response.status(),
        response_helper::AppResponse::<()> {
            message: "Something went wrong".to_string(),
            data: None,
            errors: None,
        },
    );
    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, new_response).map_into_right_body(),
    ))
}
