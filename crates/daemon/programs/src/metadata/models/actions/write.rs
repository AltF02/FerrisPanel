use crate::metadata::models::actions::Action;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::Write as IOWrite;

#[derive(Serialize, Deserialize)]
pub struct Write {
    pub target: String,
    pub content: String,
}

impl Write {}

#[typetag::serde(name = "write")]
impl Action for Write {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut file = fs::File::create(self.target.as_str())?;
        file.write_all(self.content.as_bytes())?;
        Ok(())
    }
}
