#[macro_use]
extern crate async_trait;

pub mod node;
pub mod server;
pub mod user;

pub trait Model {
    fn new() -> Self;
}

pub mod prelude {
    pub use crate::user::*;
}
