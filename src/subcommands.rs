use clap::ArgMatches;
use std::error::Error;

mod start;
mod shutdown;

pub async fn handle(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.subcommand() {
        Some(("start", start_matches)) => start::run(start_matches).await?,
        Some(("shutdown", shutdown_matches)) => shutdown::run(shutdown_matches).await?,
        None => println!("Invalid argument"),
        _ => unreachable!(),
    };

    Ok(())
}