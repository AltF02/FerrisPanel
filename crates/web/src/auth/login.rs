use actix_identity::Identity;
use actix_web::{web, Error, HttpResponse};
use core::error::HttpError;
use models::prelude::{User, UserModify};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct Login {
    id: String,
    password: String,
}

pub async fn post(
    id: Identity,
    login: web::Json<Login>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let user: Option<User> = User::login(&login.id, &login.password, &pool)
        .await
        .unwrap();

    return if let Some(user) = user {
        id.remember(user.email);
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().json(HttpError {
            info: "Unauthorized",
        }))
    };
}
