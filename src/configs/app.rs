use actix_web::dev;
use actix_web::middleware::ErrorHandlerResponse;

pub fn error_404_handler<B, E>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, E> {
    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
