use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Quote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinPaprikaTicker {
    pub id: String,

    pub name: String,

    pub symbol: String,

    pub rank: i64,

    pub total_supply: f64,

    pub max_supply: f64,

    pub beta_value: f64,

    pub first_data_at: String,

    pub last_updated: String,

    pub quotes: HashMap<String, Quote>,
}
