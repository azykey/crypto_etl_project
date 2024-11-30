use anyhow::{Context, Result};
use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub api_base_url: String,
    pub coins_to_track: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .context("DATABASE_URL must be set")?,
            api_base_url: env::var("API_BASE_URL")
                .unwrap_or_else(|_| "https://api.coingecko.com/api/v3".to_string()),
            coins_to_track: env::var("COINS_TO_TRACK")
                .unwrap_or_else(|_| "bitcoin,ethereum,binancecoin".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        })
    }
}