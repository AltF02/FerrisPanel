use clap::ArgMatches;
use std::error::Error;

#[cfg(not(debug_assertions))]
use crate::constants::PID_FILE;
#[cfg(not(debug_assertions))]
use std::fs::File;
#[cfg(not(debug_assertions))]
use std::io::prelude::*;

pub async fn run(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    #[cfg(not(debug_assertions))]
    {
        if File::open(PID_FILE).is_ok() {
            println!("Process already running...");
            std::process::exit(1);
        }

        let mut pid_file = File::create(PID_FILE)?;
        pid_file.write_all(format!("{}", std::process::id()).as_bytes())?;
    }
    println!("Starting server..");
    let connection = core::controller::connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    core::server::start(connection).await?;
    Ok(())
}
