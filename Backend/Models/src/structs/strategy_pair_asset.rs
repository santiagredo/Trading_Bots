use serde::{Deserialize, Serialize};

use crate::entities::{pair_assets, strategies};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StrategyPairAsset {
    // pub strategy_id: i32,
    pub strategy: strategies::Model,
    // pub pair_asset_id: i32,
    pub pair_asset: pair_assets::Model,
}
