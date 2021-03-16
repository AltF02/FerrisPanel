use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::error::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[async_trait]
pub trait ServerExt {
    async fn create(
        name: String,
        description: String,
        pool: &PgPool,
    ) -> Result<Option<Server>, Box<dyn Error>>;
}

#[async_trait]
impl ServerExt for Server {
    async fn create(
        name: String,
        description: String,
        pool: &PgPool,
    ) -> Result<Option<Server>, Box<dyn Error>> {
        let uuid: Option<(Uuid,)> =
            sqlx::query_as("INSERT INTO servers (name, description) VALUES ($1, $2) RETURNING id")
                .bind(&name)
                .bind(&description)
                .fetch_optional(pool)
                .await?;

        return if let Some(uuid) = uuid {
            Ok(Some(Self {
                id: uuid.0,
                name,
                description,
            }))
        } else {
            Ok(None)
        };
    }
}
