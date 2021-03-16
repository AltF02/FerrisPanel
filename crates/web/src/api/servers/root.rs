use actix_identity::Identity;
use actix_web::{web, Error, HttpResponse};
use models::server::{Server, ServerExt};
use serde::Deserialize;
use sqlx::PgPool;

pub async fn get(_id: Identity, _pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct ServerData {
    name: String,
    description: String,
}

pub async fn post(
    pool: web::Data<PgPool>,
    data: web::Json<ServerData>,
) -> Result<HttpResponse, Error> {
    let server: Option<Server> = Server::create(data.name.clone(), data.description.clone(), &pool)
        .await
        .unwrap();
    if let Some(server) = server {
        Ok(HttpResponse::Ok().json(server))
    } else {
        Ok(HttpResponse::Conflict().finish())
    }
}
