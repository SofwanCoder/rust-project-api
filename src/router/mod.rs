mod v1;
use actix_web::web;

pub(crate) fn get_router_scope() -> actix_web::Scope {
    web::scope("").service(v1::get_web_scope())
}
