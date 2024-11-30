use crate::models::CryptoCoin;
use anyhow::Result;

pub fn transform_crypto_data(coins: Vec<CryptoCoin>) -> Result<Vec<CryptoCoin>> {
    let transformed_coins: Vec<CryptoCoin> = coins
        .into_iter()
        .filter(|coin| {
            coin.market_cap > 0.0 && 
            coin.current_price > 0.0 && 
            coin.symbol.len() > 0
        })
        .map(|mut coin| {
            coin.current_price = coin.current_price.round() * 100.0 / 100.0;
            coin.market_cap = coin.market_cap.round();
            coin
        })
        .collect();

    Ok(transformed_coins)
}