use actix_web::{web, Scope};

const SCOPE: &str = "users";
pub fn get_routes() -> Scope {
    web::scope(SCOPE)
        .route(
            "{user_id}",
            web::get()
                .to(crate::controllers::users::fetch_user)
                .wrap(crate::middlewares::permit::Permission::default()),
        )
        .route("", web::post().to(crate::controllers::users::create))
}
