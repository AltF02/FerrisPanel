use crate::metadata::models::actions::Action;
use std::error::Error;
use std::fs;
use std::io::Write as IOWrite;
use std::process::Command;

pub struct Write {
    pub target: &'static str,
    pub content: String,
}

impl Write {}

impl Action for Write {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let file = fs::File::open(self.target);
        file.write_all(self.content.as_bytes())?;
        Ok(())
    }
}
