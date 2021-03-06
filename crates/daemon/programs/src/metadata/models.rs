use super::models::actions::r#move::Move;
use crate::metadata::models::actions::write::Write;
use crate::metadata::models::actions::Action;
use crate::metadata::models::data::Data;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use strfmt::Format;

pub mod actions;
pub mod data;
pub mod run;

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub setup: Vec<Box<dyn Action>>,
    pub data: Vec<Data>,
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub metadata: MetaData,
    pub vars: HashMap<String, String>,
}

impl MetaData {
    pub fn new(location: String) -> Self {
        match Self::retrieve(location) {
            Some(conf) => conf,
            None => {
                panic!("Unable to read preset file!")
            }
        }
    }

    fn retrieve(location: String) -> Option<Self> {
        match File::open(&location) {
            Ok(mut file) => {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_err() {
                    return None;
                };

                match serde_yaml::from_str(&contents) {
                    Ok(des) => Some(des),
                    Err(e) => {
                        println!("{:?}", e);
                        None
                    }
                }
            }
            Err(_) => None,
        }
    }
}

impl Program {
    pub fn new(preset: String) -> Self {
        Program {
            metadata: MetaData::new(preset),
            vars: HashMap::new(),
        }
    }

    pub fn run_setup(&self) {
        for action in self.metadata.setup.iter() {
            action.run();
        }
    }

    fn parse_data(&mut self) {
        for var in self.metadata.data.iter() {
            self.vars.insert(var.name, var.value)
        }
    }

    pub fn parse(&mut self) {
        let value = serde_yaml::to_string(&self.metadata).unwrap();
        let parsed = value.format(&self.vars).unwrap();
        let parsed_object: MetaData = serde_yaml::from_str(&parsed).unwrap();
        self.metadata = parsed_object;
    }
}
