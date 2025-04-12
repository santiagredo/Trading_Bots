use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default, Clone, Serialize)]
pub struct Ticker {
    #[serde(rename = "e")]
    pub event_type: String, // Event type

    #[serde(rename = "E")]
    pub event_time: u64, // Event time (timestamp)

    #[serde(rename = "s")]
    pub symbol: String, // Trading pair (e.g., "BNBBTC")

    #[serde(rename = "p")]
    pub price_change: Decimal, // Price change in the last 24h

    #[serde(rename = "P")]
    pub price_change_percent: Decimal, // Price change percentage

    #[serde(rename = "w")]
    pub weighted_avg_price: Decimal, // Weighted average price

    #[serde(rename = "x")]
    pub first_trade_before_24h: Decimal, // Price of the first trade before 24h window

    #[serde(rename = "c")]
    pub last_price: Decimal, // Last price

    #[serde(rename = "Q")]
    pub last_quantity: Decimal, // Last trade quantity

    #[serde(rename = "b")]
    pub best_bid_price: Decimal, // Highest bid price

    #[serde(rename = "B")]
    pub best_bid_quantity: Decimal, // Highest bid quantity

    #[serde(rename = "a")]
    pub best_ask_price: Decimal, // Lowest ask price

    #[serde(rename = "A")]
    pub best_ask_quantity: Decimal, // Lowest ask quantity

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

    #[serde(rename = "O")]
    pub stats_open_time: u64, // Time when the 24h stats window opened

    #[serde(rename = "C")]
    pub stats_close_time: u64, // Time when the 24h stats window closed

    #[serde(rename = "F")]
    pub first_trade_id: u64, // First trade ID in the last 24h

    #[serde(rename = "L")]
    pub last_trade_id: u64, // Last trade ID in the last 24h

    #[serde(rename = "n")]
    pub total_trades: u64, // Total number of trades in the last 24h
}
