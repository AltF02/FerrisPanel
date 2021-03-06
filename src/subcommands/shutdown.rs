use clap::ArgMatches;
use core::config::Config;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use sysinfo::{ProcessExt, SystemExt};

pub async fn run(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let config = Config::new();
    let pid_file = config.folders.pid + "/server.pid";
    if let Ok(mut file) = File::open(&pid_file) {
        println!("Shutting down...");

        let mut pid = String::new();
        file.read_to_string(&mut pid)?;

        Command::new("kill")
            .args(&["-15", pid.as_str()])
            .output()
            .expect("Failed to execute kill");

        std::fs::remove_file(&pid_file)?;
    } else {
        println!("Process not running...");
    }

    Ok(())
}
