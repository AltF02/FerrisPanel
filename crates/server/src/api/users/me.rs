use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use models::prelude::{User, UserModify};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct Login {
    id: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UserResponse {
    username: String,
    ,
}

pub async fn get_me(_req: HttpRequest) -> impl Responder {
    "Penis"
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
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    };
}

// pub async fn
