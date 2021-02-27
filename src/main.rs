use std::error::Error;

use clap::{load_yaml, App as ClapApp};
use dotenv::dotenv;

mod constants;
mod subcommands;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let yaml = load_yaml!("../cli.yaml");
    let matches = ClapApp::from(yaml).get_matches();

    dotenv().ok();

    subcommands::handle(&matches).await?;

    Ok(())
}
