use super::{controllers, middlewares};
mod auths;
mod users;

use actix_web::web;

const SCOPE: &str = "v1";

pub(super) fn get_web_scope() -> actix_web::Scope {
    web::scope(SCOPE)
        .route(
            "health",
            web::get().to(controllers::health::check_health_controller),
        )
        .service(users::get_routes())
        .service(auths::get_routes())
        .route("wsocket", web::get().to(controllers::wsocket::index))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().service(get_web_scope())).await;
        let req = test::TestRequest::get().uri("/v1/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_that_we_can_route_to_users() {
        let app = test::init_service(App::new().service(get_web_scope())).await;
        let req = test::TestRequest::get()
            .uri("/v1/users/health")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_that_we_can_route_to_auths() {
        let app = test::init_service(App::new().service(get_web_scope())).await;
        let req = test::TestRequest::get()
            .uri("/v1/auths/health")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
