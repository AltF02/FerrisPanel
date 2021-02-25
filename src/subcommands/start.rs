use std::error::Error;
use clap::ArgMatches;

use std::fs::File;
use std::io::prelude::*;
use crate::constants::PID_FILE;

pub async fn run(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if File::open("/run/ferrispanel.pid").is_ok() {
        println!("Process already running...");
        std::process::exit(1);
    }

    let mut pid_file = File::create(PID_FILE)?;
    pid_file.write_all(format!("{}", std::process::id()).as_bytes())?;
    println!("Starting server..");
    core::server::start().await?;
    Ok(())
}