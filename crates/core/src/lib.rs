mod error;

pub mod models { pub use models::prelude::*; }
pub mod server { pub use server::prelude::*; }
pub mod controller { pub use controller::prelude::*; }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
