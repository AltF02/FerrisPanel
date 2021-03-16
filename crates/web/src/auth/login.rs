use actix_identity::Identity;
use actix_web::{web, Error, HttpResponse};
use core::error::HttpError;
use models::prelude::{User, UserExt};
use serde::Deserialize;
use sqlx::PgPool;
use crate::api::users::me::UserResponse;

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
        id.remember(user.email.clone());
        Ok(HttpResponse::Ok().json(UserResponse {
            username: user.name,
            email: user.email,
            id: user.id
        }))
    } else {
        Ok(HttpResponse::Unauthorized().json(HttpError {
            info: "Unauthorized",
        }))
    };
}
