use crate::Model;
use bcrypt::DEFAULT_COST;
use std::error::Error;

pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,

    pub password_hash: String,
}

#[async_trait]
trait UserModify {
    async fn set_password(&mut self, pw: &str) -> Result<(), dyn std::error::Error>;
}

#[async_trait]
impl UserModify for User {
    async fn set_password(&mut self, pw: &str) -> Result<(), dyn Error> {
        let hash = bcrypt::hash(pw, DEFAULT_COST)?;
        self.password_hash = hash;
        Ok(())
    }
}

#[async_trait]
impl Model for User {
    fn new() {}
}
