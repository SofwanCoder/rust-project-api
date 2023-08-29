mod user;
use actix_web::web;

const SCOPE: &str = "v1";

pub fn get_web_scope() -> actix_web::Scope {
    web::scope(SCOPE).service(user::get_user_routes())
}
