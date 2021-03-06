use actix_web::web;

pub mod login;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::post().to(login::post));
    // .route("/{user}", web::get().to());
}
