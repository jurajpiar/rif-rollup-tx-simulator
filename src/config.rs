use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub transaction: TransactionConfig,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub account_count: u32,
    pub enable_throttling: bool,
    pub generate_reports: bool,
    pub tps: u32,
}

#[derive(Debug, Deserialize)]
pub struct TransactionConfig {
    pub min_deposit_value: u32,
    pub max_deposit_value: u32,
    pub min_transfer_value: u32,
    pub max_transfer_value: u32,
}

impl Config {
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }
}
