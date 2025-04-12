use std::{marker::PhantomData, sync::Arc};

use models::{entities::strategies_pair_assets::Model, structs::StrategyPairAsset};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::utils::Core;

#[derive(Debug, Default)]
pub struct StrategiesPairAssets<Phase = Core> {
    pub phase: PhantomData<Phase>,
    pub model: Model,
}

pub static STRATEGIES_PAIR_ASSETS: Lazy<Arc<RwLock<Vec<StrategyPairAsset>>>> =
    Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

impl StrategiesPairAssets {
    pub async fn get_active_strategies_pair_assets() -> Vec<StrategyPairAsset> {
        let strategies_pair_assets = STRATEGIES_PAIR_ASSETS.read().await;

        strategies_pair_assets.clone()
    }

    pub async fn reload_active_strategies_pair_assets() {
        let mut strategies_pair_assets = STRATEGIES_PAIR_ASSETS.write().await;

        let active_strategies_pair_assets =
            StrategiesPairAssets::<Core>::select_active_strategies_pair_assets()
                .await
                .unwrap_or_default();

        *strategies_pair_assets = active_strategies_pair_assets;

        println!(
            "Strategies pair assets: {} \n",
            strategies_pair_assets.iter().count()
        );
    }
}
