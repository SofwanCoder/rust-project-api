use crate::middlewares::permit::Authenticated;
use actix_web::{web, Scope};

const SCOPE: &str = "auths";
pub(super) fn get_routes() -> Scope {
    web::scope(SCOPE)
        .route(
            "tokens",
            web::post().to(crate::controllers::auth::create_token_controller),
        )
        .route(
            "tokens/this",
            web::delete()
                .to(crate::controllers::auth::delete_token_controller)
                .wrap(crate::middlewares::permit::Permission::<Authenticated>::allow(1)),
        )
}
