mod handler;
mod start;

use clap::{load_yaml, App as ClapApp};
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("../cli.yaml");
    let matches = ClapApp::from(yaml).get_matches();


    if let Some(ref matches) = matches.subcommand_matches("start") {
        server::start().await?;
    }

    Ok(())
}
