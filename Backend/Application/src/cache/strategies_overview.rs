use models::structs::StrategyOverview;

use crate::{
    handler::{
        Actions, Assets, Indicators, OrderStatus, Pairs, Strategies, StrategiesOverview, Tickers,
    },
    utils::Cache,
};

impl StrategiesOverview<Cache> {
    pub async fn get_active_strategy_overview_cache(
        strategy_id: &i32,
        symbol: String,
    ) -> Option<StrategyOverview> {
        let Some(strategy) = Strategies::get_active_strategy(&strategy_id).await else {
            return None;
        };

        let Some(indicator) = Indicators::get_active_indicator(&strategy_id).await else {
            return None;
        };

        let Some(action) = Actions::get_active_action(&strategy_id).await else {
            return None;
        };

        let Some(pair) = Pairs::get_active_pair(&action.pair_id).await else {
            return None;
        };

        let Some(base_asset) = Assets::get_active_asset(&pair.base_asset_id).await else {
            return None;
        };

        if Assets::get_posting_asset(&base_asset.id)
            .await
            .is_none_or(|is_posting| is_posting)
        {
            return None;
        }

        let Some(quote_asset) = Assets::get_active_asset(&pair.quote_asset_id).await else {
            return None;
        };

        if Assets::get_posting_asset(&quote_asset.id)
            .await
            .is_none_or(|is_posting| is_posting)
        {
            return None;
        }

        let Some(ticker) = Tickers::get_ticker(symbol).await else {
            return None;
        };

        let Some(order_status) = OrderStatus::get_active_status().await else {
            return None;
        };

        let strategy_overview = StrategyOverview {
            strategy,
            indicator,
            action,
            pair,
            base_asset,
            quote_asset,
            ticker,
            order_status,
        };

        Some(strategy_overview)
    }
}
