use crate::middlewares::permit_middleware::Authenticated;
use actix_web::{web, Scope};

const SCOPE: &str = "auths";
pub(super) fn get_routes() -> Scope {
    web::scope(SCOPE)
        .route(
            "tokens",
            web::post().to(crate::controllers::auth_controller::create_token),
        )
        .route(
            "tokens/this",
            web::delete()
                .to(crate::controllers::auth_controller::delete_token)
                .wrap(crate::middlewares::permit_middleware::Permission::<
                    Authenticated,
                >::allow(1)),
        )
}
