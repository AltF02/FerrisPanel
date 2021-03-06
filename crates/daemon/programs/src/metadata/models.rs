use super::models::actions::r#move::Move;
use crate::metadata::models::actions::write::Write;
use crate::metadata::models::actions::Action;

pub mod actions;
pub mod data;
pub mod run;

pub struct MetaData {
    pub setup: Vec<Box<dyn Action>>,
}

impl MetaData {
    pub fn run_setup(&self) {
        for action in self.setup.iter() {
            action.run();
        }
    }
}
