CREATE TABLE IF NOT EXISTS crypto_prices (
    id TEXT NOT NULL,
    symbol TEXT NOT NULL,
    name TEXT NOT NULL,
    current_price DECIMAL(20, 8) NOT NULL,
    market_cap DECIMAL(20, 2) NOT NULL,
    total_volume DECIMAL(20, 2) NOT NULL,
    price_change_percentage_24h DECIMAL(10, 2) NOT NULL,
    extraction_timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (id, extraction_timestamp)
);