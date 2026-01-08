use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{
    entities::{actions, assets, indicators, pairs, strategies},
    enums::Status,
    structs::Ticker,
};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StrategyOverview {
    pub strategy: strategies::Model,
    pub indicator: indicators::Model,
    pub action: actions::Model,
    pub pair: pairs::Model,
    pub base_asset: assets::Model,
    pub quote_asset: assets::Model,
    pub ticker: Ticker,
    pub order_status: HashMap<Status, i32>,
}
