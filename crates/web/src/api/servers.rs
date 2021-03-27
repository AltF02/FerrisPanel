mod root;
mod server;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(root::get))
        .route("", web::post().to(root::post))
        .route("/{server}", web::get().to(server::get));
}
