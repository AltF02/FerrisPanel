pub mod download;
pub mod r#move;
pub mod write;

use serde::{Deserialize, Serialize};
use std::error::Error;

pub trait Action: erased_serde::Serialize {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}
