pub mod users;
pub mod servers;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/users").configure(users::init))
        .service(web::scope("/servers").configure(servers::init));
}
