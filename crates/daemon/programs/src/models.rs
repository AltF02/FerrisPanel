use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};
use strfmt::Format;

use metadata::actions::Action;
use metadata::data::Data;
use metadata::run::Run;

pub mod metadata;
pub mod program;

#[derive(Serialize, Deserialize)]
pub struct MetaData {
    pub setup: Vec<Box<dyn Action>>,
    pub data: Vec<Data>,
    pub run: Run,
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
