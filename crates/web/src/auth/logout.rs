use actix_identity::Identity;
use actix_web::{web, Error, HttpResponse};

pub async fn post(id: Identity) -> Result<HttpResponse, Error> {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}
