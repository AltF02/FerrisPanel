pub mod users;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").configure(users::init));
}
