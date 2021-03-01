use clap::ArgMatches;
use std::error::Error;

mod shutdown;
mod start;
mod users;

pub async fn handle(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.subcommand() {
        Some(("start", start_matches)) => start::run(start_matches).await?,
        Some(("shutdown", shutdown_matches)) => shutdown::run(shutdown_matches).await?,
        Some(("users", users_matchers)) => users::run(users_matchers).await?,
        None => println!("Invalid argument"),
        _ => unreachable!(),
    };

    Ok(())
}
