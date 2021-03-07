pub mod download;
pub mod r#move;
pub mod write;

use std::error::Error;

#[typetag::serde(tag = "type")]
pub trait Action {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}
