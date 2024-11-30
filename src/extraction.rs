use crate::models::CryptoCoin;
use crate::config::Config;
use anyhow::Result;
use reqwest::Client;
use chrono::Utc;

pub async fn extract_crypto_data(config: &Config) -> Result<Vec<CryptoCoin>> {
    let client = Client::new();
    let url = format!(
        "{}/coins/markets?vs_currency=usd&ids={}&order=market_cap_desc&sparkline=false",
        config.api_base_url,
        config.coins_to_track.join(",")
    );

    let coins: Vec<CryptoCoin> = client.get(&url)
        .send()
        .await?
        .json()
        .await?
        .into_iter()
        .map(|mut coin: CryptoCoin| {
            coin.extraction_timestamp = Utc::now();
            coin
        })
        .collect();

    Ok(coins)
}

