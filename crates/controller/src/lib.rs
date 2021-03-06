use core::config::Config;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Pool, Postgres};
use std::error::Error;

pub async fn connect(config: &Config) -> Result<Pool<Postgres>, Box<dyn Error + Send + Sync>> {
    let options = PgConnectOptions::new()
        .host(config.database.host.as_str())
        .port(config.database.port)
        .username(config.database.user.as_str())
        .password(config.database.password.as_str())
        .database(config.database.database.as_str());

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect_with(options)
        .await?;

    Ok(pool)
}

pub mod prelude {
    pub use crate::connect;
}
