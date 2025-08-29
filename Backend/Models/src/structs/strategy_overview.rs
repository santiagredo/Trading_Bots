use serde::{Deserialize, Serialize};

use crate::{
    entities::{actions, assets, indicators, pairs, strategies},
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
}
