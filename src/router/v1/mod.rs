mod auths;
mod users;

use actix_web::web;

const SCOPE: &str = "v1";

pub fn get_web_scope() -> actix_web::Scope {
    web::scope(SCOPE)
        .service(users::get_routes())
        .service(auths::get_routes())
        .route(
            "/wsocket",
            web::get().to(crate::controllers::wsocket::index),
        )
}
