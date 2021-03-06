use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;

const DEFAULT_LOCATION: &str = "./config.yml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub folders: FolderConfig,
    pub server: ServerConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FolderConfig {
    pub pid: String,
    pub work: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub cookie_name: String,
}

impl Config {
    pub fn new() -> Config {
        match Config::retrieve() {
            Some(conf) => conf,
            None => {
                panic!("Unable to read config file!")
            }
        }
    }

    fn retrieve() -> Option<Config> {
        let location = env::var("CONFIG_PATH").unwrap_or_else(|_| DEFAULT_LOCATION.to_string());
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
