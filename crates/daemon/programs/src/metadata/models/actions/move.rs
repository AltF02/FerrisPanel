use crate::metadata::models::actions::Action;
use std::error::Error;
use std::process::Command;

pub struct Move {
    pub source: &'static str,
    pub target: &'static str,
}

impl Move {}

impl Action for Move {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        Command::new("/usr/bin/mv")
            .args(&[self.source, self.target])
            .output()?;
        Ok(())
    }
}
