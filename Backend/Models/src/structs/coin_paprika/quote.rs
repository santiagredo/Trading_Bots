use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub price: f64,

    #[serde(rename = "volume_24h")]
    pub volume_24_h: f64,

    #[serde(rename = "volume_24h_change_24h")]
    pub volume_24_h_change_24_h: f64,

    pub market_cap: i64,

    #[serde(rename = "market_cap_change_24h")]
    pub market_cap_change_24_h: f64,

    #[serde(rename = "percent_change_15m")]
    pub percent_change_15_m: f64,

    #[serde(rename = "percent_change_30m")]
    pub percent_change_30_m: f64,

    #[serde(rename = "percent_change_1h")]
    pub percent_change_1_h: f64,

    #[serde(rename = "percent_change_6h")]
    pub percent_change_6_h: f64,

    #[serde(rename = "percent_change_12h")]
    pub percent_change_12_h: f64,

    #[serde(rename = "percent_change_24h")]
    pub percent_change_24_h: f64,

    #[serde(rename = "percent_change_7d")]
    pub percent_change_7_d: f64,

    #[serde(rename = "percent_change_30d")]
    pub percent_change_30_d: f64,

    #[serde(rename = "percent_change_1y")]
    pub percent_change_1_y: f64,

    pub ath_price: f64,

    pub ath_date: String,

    pub percent_from_price_ath: f64,
}
