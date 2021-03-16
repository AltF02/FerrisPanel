#[macro_use]
extern crate async_trait;

pub mod server;
pub mod user;


pub mod prelude {
    pub use crate::user::*;
}
