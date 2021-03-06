use crate::config::Config;
use chrono::Local;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn create(config: &Config) -> Result<String, Box<dyn Error>> {
    let folder = format!("{}/logs", config.folders.work);

    if !Path::new(&folder).exists() {
        let builder = fs::DirBuilder::new();
        builder.create(&folder)?;
    }

    let filename = Local::now().format("%F-%H:%I").to_string();

    Ok(format!("{}/{}.log", &folder, filename))
}
