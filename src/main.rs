#![feature(num_as_ne_bytes)]

use std::error::Error;

use clap::{App as ClapApp, load_yaml, ArgMatches};

mod handler;
mod subcommands;
mod constants;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("../cli.yaml");
    let matches = ClapApp::from(yaml).get_matches();

    subcommands::handle(&matches).await?;

    Ok(())
}
