#[macro_use]
extern crate async_trait;

mod node;
mod server;
pub mod user;

pub trait Model {
    fn new() -> Self;
}

pub mod prelude {
    pub use crate::user::*;
}
