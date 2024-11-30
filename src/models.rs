use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CryptoCoin {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub market_cap: f64,
    pub total_volume: f64,
    pub price_change_percentage_24h: f64,
    pub extraction_timestamp: DateTime<Utc>,
}