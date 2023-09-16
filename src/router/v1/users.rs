use actix_web::{web, Scope};

const SCOPE: &str = "users";
pub fn get_routes() -> Scope {
    web::scope(SCOPE)
        .service(
            web::resource("")
                .route(web::post().to(crate::controllers::users::create_user_controller))
                .route(
                    web::get()
                        .to(crate::controllers::users::fetch_users_controller)
                        .wrap(crate::middlewares::permit::Permission::default()),
                ),
        )
        .route(
            "me",
            web::get()
                .to(crate::controllers::users::fetch_me_controller)
                .wrap(crate::middlewares::permit::Permission::default()),
        )
        .service(
            web::scope("{user_id}")
                .service(
                    web::resource("")
                        .route(web::put().to(crate::controllers::users::update_user_controller))
                        .route(web::get().to(crate::controllers::users::fetch_user_controller)),
                )
                .route(
                    "password",
                    web::put().to(crate::controllers::users::update_password_controller),
                )
                .wrap(crate::middlewares::permit::Permission::default()),
        )
}
