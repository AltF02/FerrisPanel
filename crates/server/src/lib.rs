#[macro_use]
extern crate actix_web;

use actix_web::{HttpServer, App, HttpRequest, Responder};
use std::error::Error;

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello from the index page!"
}

pub async fn start() -> Result<(), Box<dyn Error>>{
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await?;

    Ok(())
}