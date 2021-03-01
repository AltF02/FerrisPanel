use crate::constants::PID_FILE;
use clap::ArgMatches;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process::Command;

pub async fn run(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if let Ok(mut file) = File::open(PID_FILE) {
        println!("Shutting down...");

        let mut pid = String::new();
        file.read_to_string(&mut pid)?;

        Command::new("kill")
            .args(&["-9", pid.as_str()])
            .output()
            .expect("Failed to execute kill");

        std::fs::remove_file(PID_FILE)?;
    } else {
        println!("Process not running...");
    }

    Ok(())
}
