use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use core::error::HttpError;
use models::prelude::{User, UserModify};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    username: String,
    email: String,
    id: i32,
}

pub async fn get(id: Identity, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    return if let Some(user_id) = id.identity() {
        let user: User = User::fetch(user_id, &pool).await.unwrap().unwrap();
        Ok(HttpResponse::Ok().json(UserResponse {
            username: user.name,
            email: user.email,
            id: user.id,
        }))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    };
}

// pub async fn
