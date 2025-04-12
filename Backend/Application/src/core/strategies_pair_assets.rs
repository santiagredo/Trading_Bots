use models::{entities::pair_assets, structs::StrategyPairAsset};

use crate::{
    config::get_config,
    types::{PairAssets, Strategies, StrategiesPairAssets},
    utils::{Core, Data, Outcome},
};

impl StrategiesPairAssets<Core> {
    pub async fn select_active_strategies_pair_assets(
    ) -> Outcome<Vec<StrategyPairAsset>, String, String> {
        let active_strategies = Strategies::<Core>::select_active_strategies()
            .await
            .unwrap_or_default();

        let strats_pair_assets =
            StrategiesPairAssets::<Data>::select_active_strategies_pair_assets(
                &get_config().await.db,
                &active_strategies,
            )
            .await?;

        let pair_asset_ids = strats_pair_assets
            .iter()
            .map(|val| val.pair_asset_id)
            .collect::<Vec<i32>>();

        let pair_assets = PairAssets::<Core>::select_all_pair_assets()
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|val| pair_asset_ids.contains(&val.id))
            .collect::<Vec<pair_assets::Model>>();

        let strategies_pair_assets = strats_pair_assets
            .into_iter()
            .map(|spa| StrategyPairAsset {
                strategy: active_strategies
                    .iter()
                    .find(|strat| strat.id == spa.strategy_id)
                    .cloned()
                    .unwrap_or_default(),
                pair_asset: pair_assets
                    .iter()
                    .find(|pa| pa.id == spa.pair_asset_id)
                    .cloned()
                    .unwrap_or_default(),
            })
            .collect();

        Ok(strategies_pair_assets)
    }
}
