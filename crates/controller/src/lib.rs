use sqlx::{Postgres, Pool};
use sqlx::postgres::PgPoolOptions;
use std::error::Error;

pub mod users;

pub async fn connect(
    uri: &str,
) -> Result<Pool<Postgres>, Box<dyn Error + Send + Sync>> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(uri)
        .await?;

    Ok(pool)
}

pub mod prelude {
    pub use crate::users;
    pub use crate::connect;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
