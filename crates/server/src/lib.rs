#[macro_use]
extern crate actix_web;

use actix_web::{HttpServer, App, HttpRequest, Responder};
use std::error::Error;

pub mod prelude {
    pub use crate::start;
}

#[get("/test")]
async fn pee(_req: HttpRequest) -> impl Responder {
    "Penis"
}

pub async fn start() -> Result<(), Box<dyn Error>>{
    HttpServer::new(|| {
        App::new()
            .service(pee)
            .service(actix_files::Files::new("/", "./client/dist").index_file("index.html"))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await?;

    Ok(())
}