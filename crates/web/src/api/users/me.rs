use actix_identity::Identity;
use actix_web::{web, Error, HttpResponse};
use models::prelude::{User, UserModify};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) id: i32,
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
