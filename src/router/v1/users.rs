use actix_web::{web, Scope};

const SCOPE: &str = "users";
pub fn get_routes() -> Scope {
    web::scope(SCOPE)
        .route(
            "",
            web::get()
                .to(crate::controllers::users::fetch)
                .wrap(crate::middlewares::permit::Permission::allow(1)),
        )
        .route("", web::post().to(crate::controllers::users::create))
}
