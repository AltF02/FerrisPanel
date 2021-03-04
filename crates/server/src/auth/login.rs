use actix_identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use core::error::HttpError;
use models::prelude::{User, UserModify};
use serde::{Deserialize, Serialize};
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
        id.remember(user.email.to_owned());
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().json(HttpError {
            info: "Unauthorized",
        }))
    };
}
