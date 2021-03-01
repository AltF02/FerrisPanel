use actix_web::web;

mod me;
mod user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/me", web::get().to(me::get_me))
        .route("/me", web::post().to(me::post_me));
}
