use actix_identity::Identity;
use actix_web::{web, Error, HttpResponse};
use sqlx::PgPool;

pub async fn get(_id: Identity, _pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}
