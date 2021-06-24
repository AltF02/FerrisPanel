use crate::models::metadata::actions::Action;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct Download {
    pub url: String,
    pub target: String,
}

impl Download {}

#[typetag::serde(name = "download")]
impl Action for Download {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        Command::new("curl")
            .args(&[&self.url, "-o", &self.target])
            .output()?;

        Ok(())
    }
}
