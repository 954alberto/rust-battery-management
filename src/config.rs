use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub settings: Settings,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub capacity: f64,
    pub initial_charge: f64,
    pub max_rate: f64,
    pub efficiency: f64,
    pub grid_limit: f64,
}

pub fn load_config(file_path: &str) -> Result<Config> {
    let data = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read configuration file: {}", file_path))?;

    let config: Config =
        toml::de::from_str(&data).with_context(|| "Failed to parse configuration file")?;

    Ok(config)
}
