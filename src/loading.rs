use crate::models::CryptoCoin;
use anyhow::Result;
use sqlx::PgPool;

pub async fn load_to_postgres(pool: &PgPool, coins: &[CryptoCoin]) -> Result<()> {
    for coin in coins {
        sqlx::query!(
            r#"
            INSERT INTO crypto_prices 
            (id, symbol, name, current_price, market_cap, total_volume, 
             price_change_percentage_24h, extraction_timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (id, extraction_timestamp) DO UPDATE 
            SET 
                current_price = EXCLUDED.current_price,
                market_cap = EXCLUDED.market_cap,
                total_volume = EXCLUDED.total_volume,
                price_change_percentage_24h = EXCLUDED.price_change_percentage_24h
            "#,
            coin.id,
            coin.symbol,
            coin.name,
            coin.current_price,
            coin.market_cap,
            coin.total_volume,
            coin.price_change_percentage_24h,
            coin.extraction_timestamp
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}