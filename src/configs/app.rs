use crate::{configs::constant, helpers::response::AppResponse};
use actix_web::{dev, dev::ServiceResponse, middleware::ErrorHandlerResponse};

pub fn error_404_handler<B, E>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, E> {
    let status = res.status();
    let request = res.into_parts().0;

    let new_response =
        AppResponse::Error::<()>("Resource not found", None).to_http_response(status);

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

    let new_response =
        AppResponse::Error::<()>("Something went wrong", None).to_http_response(response.status());
    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, new_response).map_into_right_body(),
    ))
}
