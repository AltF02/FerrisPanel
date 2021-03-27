use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Run {
    pub program: String,
    pub args: Vec<String>,
}
