use super::{controllers, middlewares};
use actix_web::{web, Scope};

const SCOPE: &str = "users";
pub(super) fn get_routes() -> Scope {
    web::scope(SCOPE)
        .route("health", web::get().to(actix_web::HttpResponse::Ok))
        .service(
            web::resource("")
                .route(web::post().to(controllers::user::create_user_controller))
                .route(
                    web::get()
                        .to(controllers::user::fetch_users_controller)
                        .wrap(middlewares::permit::Permission::default()),
                ),
        )
        .route(
            "me",
            web::get()
                .to(controllers::user::fetch_me_controller)
                .wrap(middlewares::permit::Permission::default()),
        )
        .service(
            web::scope("{user_id}")
                .service(
                    web::resource("")
                        .route(web::put().to(controllers::user::update_user_controller))
                        .route(web::get().to(controllers::user::fetch_user_controller)),
                )
                .route(
                    "password",
                    web::put().to(controllers::user::update_password_controller),
                )
                .wrap(middlewares::permit::Permission::default()),
        )
}
