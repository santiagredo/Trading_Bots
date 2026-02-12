use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    entities::{actions, assets, indicators, pairs, status, strategies},
    structs::Ticker,
};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StrategyOverview {
    pub strategy: strategies::Model,
    pub indicators: Vec<indicators::Model>,
    pub action: actions::Model,
    pub pair: pairs::Model,
    pub base_asset: assets::Model,
    pub quote_asset: assets::Model,
    pub ticker: Ticker,
    pub order_status: HashMap<i32, status::Model>,
}
