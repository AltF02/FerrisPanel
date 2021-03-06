pub mod download;
pub mod r#move;
pub mod write;

use std::error::Error;

pub trait Action {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}
