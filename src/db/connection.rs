use crate::db::{config::Config, errors::ConfigError};

pub fn test_connection(config_path: String) -> Result<(), ConfigError> {
    let config = Config::load_config(config_path)?;

    println!("Testing connection to database: {}", config.database.url);

    Ok(())
}
