use actix_web::{web, HttpResponse, Scope};

const SCOPE: &str = "users";
pub fn get_user_routes() -> Scope {
    web::scope(SCOPE)
        .route(
            "",
            web::get()
                .to(|| HttpResponse::Ok())
                .wrap(crate::middlewares::permit::Permission::allow(1)),
        )
        .route(
            "",
            web::post()
                .to(crate::controllers::user::create)
                .wrap(crate::middlewares::permit::Permission::allow(1)),
        )
}
