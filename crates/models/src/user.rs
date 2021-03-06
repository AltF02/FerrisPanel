use bcrypt::DEFAULT_COST;
use sqlx::PgPool;
use std::error::Error;

#[derive(sqlx::FromRow, Clone, Debug)]
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
    async fn fetch(fetch: String, pool: &PgPool) -> Result<Option<User>, Box<dyn Error>>;
    async fn login(id: &str, password: &str, pool: &PgPool)
        -> Result<Option<User>, Box<dyn Error>>;
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

    async fn fetch(fetch: String, pool: &PgPool) -> Result<Option<User>, Box<dyn Error>> {
        let user: Option<User> =
            sqlx::query_as("SELECT * FROM users WHERE email = $1 OR name = $1")
                .bind(fetch)
                .fetch_optional(pool)
                .await?;

        Ok(user)
    }

    async fn login(
        id: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Option<User>, Box<dyn Error>> {
        let user: Option<User> =
            sqlx::query_as("SELECT * FROM users WHERE email = $1 OR name = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?;

        if user.is_none() {
            return Ok(None);
        }

        let user_hash = user.clone().unwrap().password_hash;
        let verify = bcrypt::verify(password, user_hash.as_str());
        if verify.unwrap() {
            return Ok(user);
        }

        Ok(None)
    }
}
