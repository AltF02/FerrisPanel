use clap::ArgMatches;
use core::config::Config;
use std::error::Error;

#[cfg(not(debug_assertions))]
use std::fs::File;
#[cfg(not(debug_assertions))]
use std::io::prelude::*;

pub async fn run(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    #[cfg(not(debug_assertions))]
    {
        let config = Config::new();
        let pid_file = config.folders.pid + "/server.pid";
        if let Ok(_pid) = File::open(&pid_file) {
            println!("Process already running...");
            std::process::exit(1);
        }

        let mut pid_file = File::create(&pid_file)?;
        pid_file.write_all(format!("{}", std::process::id()).as_bytes())?;
    }
    println!("Starting server..");
    server::start().await?;
    Ok(())
}
