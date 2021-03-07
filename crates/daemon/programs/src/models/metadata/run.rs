use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process::{Child, Command, Output};

#[derive(Serialize, Deserialize)]
pub struct Run {
    pub program: String,
    pub args: Vec<String>,
}
