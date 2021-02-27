use bcrypt::DEFAULT_COST;
use sqlx::PgPool;
use std::error::Error;

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,

    pub password_hash: String,
}

#[async_trait]
pub trait UserModify {
    async fn create(
        email: String,
        name: String,
        pw: String,
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>>;
    async fn set_password(&mut self, pw: String, pool: &PgPool) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
impl UserModify for User {
    async fn create(
        name: String,
        email: String,
        pw: String,
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let hash = bcrypt::hash(pw, DEFAULT_COST)?;

        sqlx::query("INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3)")
            .bind(name)
            .bind(email)
            .bind(hash)
            .execute(&pool)
            .await?;

        Ok(())
    }

    async fn set_password(&mut self, pw: String, pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let hash = bcrypt::hash(pw, DEFAULT_COST)?;
        self.password_hash = hash.clone();
        sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
            .bind(self.id)
            .bind(hash.clone())
            .execute(pool)
            .await?;

        Ok(())
    }
}

// impl Model for User {
//     fn new() {
//         return;
//     }
// }
