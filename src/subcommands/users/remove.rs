use clap::ArgMatches;
use std::error::Error;

pub(crate) async fn run(_matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    Ok(())
}
