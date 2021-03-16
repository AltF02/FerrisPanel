mod root;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(root::get))
        .route("", web::post().to(root::post));
    // .route("/{user}", web::get().to());
}
