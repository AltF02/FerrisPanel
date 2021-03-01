use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use models::prelude::{User, UserModify};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct Login {
    id: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    username: String,
    email: String,
    id: i32,
}

pub async fn get_me(id: Identity, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
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

pub async fn post_me(
    id: Identity,
    login: web::Json<Login>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let user: Option<User> = User::login(&login.id, &login.password, &pool)
        .await
        .unwrap();

    return if let Some(user) = user {
        id.remember(user.email.to_owned());
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
