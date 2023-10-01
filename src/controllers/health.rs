use crate::helpers::response;
use actix_web::{HttpRequest, Responder, Result};
use common::helpers::error::AppError;
use tracing::instrument;

#[instrument(skip_all)]
pub async fn check_health_controller(req: HttpRequest) -> Result<impl Responder, AppError> {
    let _ = req.app_data::<crate::ApplicationContext>();

    let result = Result::<(), AppError>::Ok(());

    result.map(response::ok)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_health_check() {
        let req = test::TestRequest::get().uri("/health").to_http_request();
        let resp = check_health_controller(req).await;
        assert_eq!(resp.is_ok(), true);
    }
}
