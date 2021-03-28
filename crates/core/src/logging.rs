use crate::config::Config;
use log::LevelFilter;
use std::error::Error;

pub fn setup(_config: &Config) -> Result<(), Box<dyn Error>> {
    #[cfg(not(debug_assertions))]
    let level = LevelFilter::Info;
    #[cfg(debug_assertions)]
    let level = LevelFilter::Debug;

    let logger = fern::Dispatch::new()
        .level(level)
        .level_for("sqlx", LevelFilter::Warn)
        .chain(std::io::stdout());

    #[cfg(not(debug_assertions))]
    logger.chain(fern::DateBased::new(
        format!("{}/logs", _config.folders.work),
        "%F-%H:%I.log",
    ));

    logger.apply()?;

    Ok(())
}
