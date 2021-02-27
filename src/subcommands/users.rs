use clap::ArgMatches;
use std::error::Error;

mod add;
mod remove;

pub async fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    if let Some(ref _matches) = matches.subcommand_matches("add") {
        add::run().await?
    } else if let Some(ref matches) = matches.subcommand_matches("remove") {
        remove::run(matches).await?;
    }

    Ok(())
}
