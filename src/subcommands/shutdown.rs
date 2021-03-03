use clap::ArgMatches;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process::Command;

pub async fn run(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let pid_file = std::env::var("PID_FOLDER")? + "/server.pid";
    if let Ok(mut file) = File::open(&pid_file) {
        println!("Shutting down...");

        let mut pid = String::new();
        file.read_to_string(&mut pid)?;

        Command::new("kill")
            .args(&["-9", pid.as_str()])
            .output()
            .expect("Failed to execute kill");

        std::fs::remove_file(&pid_file)?;
    } else {
        println!("Process not running...");
    }

    Ok(())
}
