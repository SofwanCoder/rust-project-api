mod auths;
mod users;

use actix_web::web;

const SCOPE: &str = "v1";

pub(super) fn get_web_scope() -> actix_web::Scope {
    web::scope(SCOPE)
        .route(
            "health",
            web::get().to(crate::controllers::health::check_health_controller),
        )
        .service(users::get_routes())
        .service(auths::get_routes())
        .route(
            "/wsocket",
            web::get().to(crate::controllers::wsocket::index),
        )
}
