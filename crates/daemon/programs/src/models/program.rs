use crate::models::MetaData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Child;
use strfmt::Format;

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub metadata: MetaData,
    pub vars: HashMap<String, String>,
    pub pid: Option<u32>,
}

impl Program {
    pub fn new(preset: String) -> Self {
        Program {
            metadata: MetaData::new(preset),
            vars: HashMap::new(),
            pid: None,
        }
    }

    pub fn run_setup(&self) {
        for action in self.metadata.setup.iter() {
            action.run().expect("Unable to run action");
        }
    }

    fn parse_data(&mut self) {
        for var in self.metadata.data.iter() {
            self.vars.insert(var.name.clone(), var.value.clone());
        }
    }

    pub fn parse(&mut self) {
        self.parse_data();
        let value = serde_yaml::to_string(&self.metadata).unwrap();
        let parsed = value.format(&self.vars).unwrap();
        let parsed_object: MetaData = serde_yaml::from_str(&parsed).unwrap();
        self.metadata = parsed_object;
    }
}
