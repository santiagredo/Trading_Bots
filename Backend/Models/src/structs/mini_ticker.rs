use sea_orm::prelude::Decimal;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default, Clone)]
pub struct MiniTicker {
    #[serde(rename = "e")]
    pub event_type: String, // Event type (e.g., "24hrMiniTicker")

    #[serde(rename = "E")]
    pub event_time: u64, // Event time (timestamp)

    #[serde(rename = "s")]
    pub symbol: String, // Trading pair (e.g., "BTCUSDT")

    #[serde(rename = "c")]
    pub last_price: Decimal, // Last price

    #[serde(rename = "o")]
    pub open_price: Decimal, // Opening price (24h ago)

    #[serde(rename = "h")]
    pub high_price: Decimal, // Highest price in the last 24h

    #[serde(rename = "l")]
    pub low_price: Decimal, // Lowest price in the last 24h

    #[serde(rename = "v")]
    pub base_asset_volume: Decimal, // Total BTC traded in the last 24h

    #[serde(rename = "q")]
    pub quote_asset_volume: Decimal, // Total USDT traded in the last 24h
}
