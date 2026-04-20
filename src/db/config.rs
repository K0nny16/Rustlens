use super::errors::ConfigError;
use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string};

use toml::from_str;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub table: TableConfig,
    pub analysis: AnalysisConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TableConfig{
    pub name: String,
    pub columns: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct AnalysisConfig {
    pub level: AnalysisLevel
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnalysisLevel {
    Quick,
    Standard,
    Deep,
}

#[derive(Debug, Deserialize)]
pub struct OutputConfig {
    pub format: OutputFormat,
    pub path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Csv,
}

impl Config {
    pub fn load_config(path: String) -> Result<Config, ConfigError> {
        let config_contents = read_to_string(path).map_err(ConfigError::ReadFile)?;
        let config: Config = from_str(&config_contents).map_err(ConfigError::ParseToml)?;

        Ok(config)
    }
}
