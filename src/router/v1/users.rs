use actix_web::{web, Scope};

const SCOPE: &str = "users";
pub(super) fn get_routes() -> Scope {
    web::scope(SCOPE)
        .service(
            web::resource("")
                .route(web::post().to(crate::controllers::user_controller::create_user_controller))
                .route(
                    web::get()
                        .to(crate::controllers::user_controller::fetch_users_controller)
                        .wrap(crate::middlewares::permit_middleware::Permission::default()),
                ),
        )
        .route(
            "me",
            web::get()
                .to(crate::controllers::user_controller::fetch_me_controller)
                .wrap(crate::middlewares::permit_middleware::Permission::default()),
        )
        .service(
            web::scope("{user_id}")
                .service(
                    web::resource("")
                        .route(
                            web::put()
                                .to(crate::controllers::user_controller::update_user_controller),
                        )
                        .route(
                            web::get()
                                .to(crate::controllers::user_controller::fetch_user_controller),
                        ),
                )
                .route(
                    "password",
                    web::put().to(crate::controllers::user_controller::update_password_controller),
                )
                .wrap(crate::middlewares::permit_middleware::Permission::default()),
        )
}
