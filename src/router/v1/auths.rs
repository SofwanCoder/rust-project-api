use actix_web::{web, Scope};

const SCOPE: &str = "auths";
pub fn get_routes() -> Scope {
    web::scope(SCOPE).route(
        "tokens",
        web::post().to(crate::controllers::auths::create_tokens),
    )
}
