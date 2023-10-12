use super::{controllers, middlewares, middlewares::permit::Authenticated};
use actix_web::{web, Scope};

const SCOPE: &str = "auths";
pub(super) fn get_routes() -> Scope {
    web::scope(SCOPE)
        .route("health", web::get().to(actix_web::HttpResponse::Ok))
        .route(
            "tokens",
            web::post().to(controllers::auth::create_token_controller),
        )
        .route(
            "tokens/this",
            web::delete()
                .to(controllers::auth::delete_token_controller)
                .wrap(middlewares::permit::Permission::<Authenticated>::allow(1)),
        )
}
