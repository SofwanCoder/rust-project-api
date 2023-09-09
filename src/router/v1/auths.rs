use crate::middlewares::permit::Authenticated;
use actix_web::{web, Scope};

const SCOPE: &str = "auths";
pub fn get_routes() -> Scope {
    web::scope(SCOPE)
        .route(
            "tokens",
            web::post().to(crate::controllers::auths::create_token),
        )
        .route(
            "tokens/this",
            web::delete()
                .to(crate::controllers::auths::delete_token)
                .wrap(crate::middlewares::permit::Permission::<Authenticated>::allow(1)),
        )
}
