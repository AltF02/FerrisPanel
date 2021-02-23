#[macro_use]
extern crate async_trait;

pub mod user;

#[async_trait]
pub trait Model {
    fn new() -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
