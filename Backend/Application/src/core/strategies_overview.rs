use models::{
    enums::Status,
    structs::{Environments, StrategyOverview, StrategyRequest},
};

use crate::{
    guard::LockSkipGuard,
    handler::{
        Actions, Assets, Indicators, OrderStatus, Orders, Pairs, Strategies, StrategiesOverview,
        Tickers, DBC,
    },
    utils::{Core, Data, Logic, Response},
};

impl StrategiesOverview<Core> {
    pub async fn select_strategy_overview_core(
        environment: Environments,
        strategy: StrategyRequest,
    ) -> Result<Vec<StrategyOverview>, Response> {
        let results = StrategiesOverview::<Data>::select_strategy_overview_data(
            &DBC::db(&environment).await?,
            strategy,
        )
        .await?;

        Ok(StrategiesOverview::<Logic>::select_strategies_overview_logic(results))
    }

    pub async fn get_strategy_overview_core(
        environment: Environments,
        strategy_id: &i32,
        symbol: String,
    ) -> Option<StrategyOverview> {
        let mut strategy_request = Strategies::default();
        strategy_request.model.id = Some(*strategy_id);
        strategy_request.environment = environment;

        let strategy = match strategy_request.get_strategy().await {
            None => return None,
            Some(val) => {
                if val.is_posting {
                    LockSkipGuard::hit(environment);
                    return None;
                }

                val.model
            }
        };

        let mut indicator_request = Indicators::default();
        indicator_request.model.strategy_id = Some(*strategy_id);
        indicator_request.environment = environment;

        let Some(indicator) = indicator_request.get_indicator().await else {
            return None;
        };

        let mut action_request = Actions::default();
        action_request.model.strategy_id = Some(*strategy_id);
        action_request.environment = environment;

        let Some(action) = action_request.get_action().await else {
            return None;
        };

        let mut pair_request = Pairs::default();
        pair_request.model.id = Some(action.pair_id);
        pair_request.environment = environment;

        let Some(pair) = pair_request.get_pair().await else {
            return None;
        };

        let mut base_asset_request = Assets::default();
        base_asset_request.model.id = Some(pair.base_asset_id);
        base_asset_request.environment = environment;

        let Some(base_asset) = base_asset_request.get_asset().await else {
            return None;
        };

        let mut quote_asset_request = Assets::default();
        quote_asset_request.model.id = Some(pair.quote_asset_id);
        quote_asset_request.environment = environment;

        let Some(quote_asset) = quote_asset_request.get_asset().await else {
            return None;
        };

        let Some(ticker) = Tickers::get_ticker(symbol).await else {
            return None;
        };

        let Some(order_status) = OrderStatus::default()
            .with_env(environment)
            .get_statuses()
            .await
        else {
            return None;
        };

        let strategy_overview = StrategyOverview {
            strategy,
            indicator,
            action,
            pair,
            base_asset: base_asset,
            quote_asset: quote_asset,
            ticker,
            order_status,
        };

        Some(strategy_overview)
    }

    pub fn evaluate_strategy_overview_core(
        strategy_overview: &StrategyOverview,
    ) -> Result<Orders, String> {
        // evaluate cooldown has passed
        if !Strategies::default().evaluate_cooldown(
            strategy_overview.strategy.last_execution,
            strategy_overview.strategy.cooldown,
        ) {
            return Err(format!("Cooldown is active"));
        }

        // evaluate error cooldown has passed
        if !Strategies::default().evaluate_cooldown(
            strategy_overview.strategy.error_last_date,
            strategy_overview.strategy.error_cooldown,
        ) {
            return Err(format!("Error cooldown is active"));
        }

        // evalute indicator
        match Indicators::evalute_indicators(
            &strategy_overview.ticker,
            &strategy_overview.indicator,
            &strategy_overview.pair,
        ) {
            Err(err) => return Err(err),
            Ok(false) => return Err(format!("Indicator not met")),
            Ok(true) => (),
        };

        // evaluate and modify action
        let action = match Actions::default().evaluate_action(
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

        let order = Orders::default()
            .from_strategy(&strategy_overview.strategy)
            .from_action(&action)
            .from_pair(&strategy_overview.pair)
            .from_ticker(&strategy_overview.ticker)
            .from_status(order_status);

        Ok(order)
    }
}
