use clap::ArgMatches;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use sysinfo::{ProcessExt, SystemExt};

pub async fn run(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let pid_file = std::env::var("PID_FOLDER")? + "/server.pid";
    if let Ok(mut file) = File::open(&pid_file) {
        println!("Shutting down...");

        let mut pid = String::new();
        file.read_to_string(&mut pid)?;
        let system = sysinfo::System::new();

        if let Some(proc) = system.get_process(pid.parse::<i32>().unwrap()) {
            proc.kill(sysinfo::Signal::Term);
        } else {
            println!("Unable to locate process, did it crash?");
        }

        std::fs::remove_file(&pid_file)?;
    } else {
        println!("Process not running...");
    }

    Ok(())
}
