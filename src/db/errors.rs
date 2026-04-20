use std::{fmt::Display, fmt::Formatter, fmt::Result, io::Error};
use toml::de::Error as TomlError;

#[derive(Debug)]
pub enum ConfigError {
    ReadFile(Error),
    ParseToml(TomlError),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ConfigError::ReadFile(e) => write!(f, "Failed to read config file: {}", e),
            ConfigError::ParseToml(e) => write!(f, "Failed to parse config file: {}", e),
        }
    }
}
