use actix_web::web;

mod me;
mod user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/me", web::get().to(me::get));
    // .route("/{user}", web::get().to());
}
