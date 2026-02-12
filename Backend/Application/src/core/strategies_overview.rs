use crate::{
    guard::LockSkipGuard,
    handler::{
        Actions, Assets, Indicators, OrderStatus, Orders, Pairs, Strategies, StrategiesOverview,
        Tickers,
    },
    logic,
    utils::EntityCache,
};
use models::{
    enums::{Status, TradingState},
    structs::{Environments, OrderRequest, StrategyOverview},
};

impl StrategiesOverview {
    pub async fn get_strategy_overview(
        environment: Environments,
        strategy_id: &i32,
        symbol: String,
    ) -> Option<StrategyOverview> {
        let strategy = match Strategies::blank().get(environment, *strategy_id).await {
            None => return None,
            Some(val) => {
                if val.state != TradingState::Ready {
                    LockSkipGuard::hit(environment);
                    return None;
                }

                val.model
            }
        };

        let Some(indicators) = Indicators::blank().get_all(environment).await else {
            return None;
        };

        let indicators = indicators
            .models
            .into_values()
            .filter(|ind| ind.strategy_id == *strategy_id)
            .collect();

        let Some(action) = Actions::blank().get(environment, *strategy_id).await else {
            return None;
        };

        let Some(pair) = Pairs::blank().get(environment, action.pair_id).await else {
            return None;
        };

        let Some(base_asset) = Assets::blank().get(environment, pair.base_asset_id).await else {
            return None;
        };

        let Some(quote_asset) = Assets::blank().get(environment, pair.quote_asset_id).await else {
            return None;
        };

        let Some(ticker) = Tickers::get_ticker(symbol).await else {
            return None;
        };

        let Some(order_status) = OrderStatus::blank().get_all(environment).await else {
            return None;
        };

        let strategy_overview = StrategyOverview {
            strategy,
            indicators,
            action,
            pair,
            base_asset: base_asset,
            quote_asset: quote_asset,
            ticker,
            order_status: order_status.models,
        };

        Some(strategy_overview)
    }

    pub fn evaluate_strategy_overview(
        strategy_overview: &StrategyOverview,
    ) -> Result<OrderRequest, String> {
        // evaluate cooldown has passed
        if !logic::strategies::evaluate_cooldown(
            strategy_overview.strategy.last_execution,
            strategy_overview.strategy.cooldown,
        ) {
            return Err(format!("Cooldown is active"));
        }

        // evaluate error cooldown has passed
        if !logic::strategies::evaluate_cooldown(
            strategy_overview.strategy.error_last_date,
            strategy_overview.strategy.error_cooldown,
        ) {
            return Err(format!("Error cooldown is active"));
        }

        // evalute indicator
        for indicator in &strategy_overview.indicators {
            match logic::indicators::evaluate_indicator(
                &indicator,
                &strategy_overview.ticker,
                &strategy_overview.pair,
            ) {
                Err(err) => return Err(err),
                Ok(false) => return Err(format!("Indicator not met")),
                Ok(true) => (),
            };
        }

        // evaluate and modify action
        let action = match Actions::blank().evaluate_action(
            strategy_overview.action.clone(),
            &strategy_overview.pair,
            &strategy_overview.ticker,
            &strategy_overview.base_asset,
            &strategy_overview.quote_asset,
        ) {
            Err(err) => return Err(err),
            Ok(val) => val,
        };

        // create order
        let order_status = strategy_overview
            .order_status
            .values()
            .find(|m| Status::Completed == Status::from_model(m))
            .cloned()
            .map(|val| val.id)
            .unwrap_or(2);

        let mut order_request = OrderRequest::default();

        Orders::from_strategy(&mut order_request, &strategy_overview.strategy);
        Orders::from_action(&mut order_request, &action);
        Orders::from_pair(&mut order_request, &strategy_overview.pair);
        Orders::from_ticker(&mut order_request, &strategy_overview.ticker);
        Orders::from_status(&mut order_request, order_status);

        Ok(order_request)
    }
}
