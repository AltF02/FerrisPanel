use actix_web::web;

pub mod login;
pub mod logout;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::post().to(login::post))
        .route("/logout", web::post().to(logout::post));
}
