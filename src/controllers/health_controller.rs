use crate::helpers::{error_helper::AppError, response_helper};
use actix_web::{HttpRequest, Responder, Result};
pub async fn check_health_controller(req: HttpRequest) -> Result<impl Responder, AppError> {
    let _ = req.app_data::<crate::ApplicationContext>().unwrap().clone();

    let result = Result::<(), AppError>::Ok(());

    result.map(response_helper::ok)
}
