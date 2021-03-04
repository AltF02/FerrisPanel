// use std::error::Error as StdError;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct Error {}

#[derive(Deserialize, Serialize)]
pub struct HttpError {
    pub info: &'static str,
}
