#[macro_use]
extern crate async_trait;

pub mod user;
mod server;
mod node;

pub trait Model {
    fn new() -> Self;
}

pub mod prelude {
    pub use crate::user::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
