#![allow(clippy::type_complexity)]

mod api;
mod auth;
mod middleware;

#[macro_use]
extern crate actix_web;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::http::ContentEncoding;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use core::config::Config;
use log::LevelFilter;
use rand::Rng;
use std::error::Error;

pub mod prelude {
    pub use crate::start;
}

#[get("/api/test")]
async fn pee(_req: HttpRequest) -> impl Responder {
    "Penis"
}

pub async fn start() -> Result<(), Box<dyn Error>> {
    let config = Config::new();
    let pg_pool = controller::connect(&config).await.unwrap();

    #[cfg(not(debug_assertions))]
    simple_logging::log_to_file(core::logging::create(&config)?, LevelFilter::Info)?;
    #[cfg(debug_assertions)]
    simple_logging::log_to_stderr(LevelFilter::Debug);

    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    HttpServer::new(move || {
        App::new()
            .data(pg_pool.clone())
            .wrap(actix_web::middleware::Compress::new(ContentEncoding::Br))
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .secure(true)
                    .name(&config.server.cookie_name),
            ))
            .service(pee)
            .service(
                web::scope("/api")
                    .wrap(middleware::authentication::Auth)
                    .configure(api::init),
            )
            .service(web::scope("/auth").configure(auth::init))
            .service(actix_files::Files::new("/", "./www").index_file("index.html"))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await?;

    Ok(())
}
