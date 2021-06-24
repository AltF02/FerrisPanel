use crate::models::metadata::actions::Action;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Move {
    pub source: String,
    pub target: String,
}

impl Move {}

#[typetag::serde(name = "move")]
impl Action for Move {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        fs::rename(&self.source, &self.target)?;
        Ok(())
    }
}
