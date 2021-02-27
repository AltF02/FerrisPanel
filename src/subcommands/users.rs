use clap::ArgMatches;
use std::error::Error;

mod add;

pub async fn run(_matches: &ArgMatches) -> Result<(), Box<dyn Error>>{
    Ok(())
}