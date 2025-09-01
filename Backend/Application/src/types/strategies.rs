use std::{collections::HashMap, marker::PhantomData, sync::Arc, time::Instant};

use chrono::{Local, NaiveDateTime};
use models::{
    entities::{
        orders,
        strategies::{self, Model},
    },
    enums::MetricType,
    structs::{AssetRequest, LedgerRequest, StrategyOverview, StrategyRequest},
};
use once_cell::sync::Lazy;
use tokio::{sync::RwLock, task::AbortHandle};

use crate::{
    types::{
        Actions, Assets, Indicators, Ledgers, Metrics, Orders, Senders, StrategiesOverview, Tickers,
    },
    utils::{Response, Types},
};

#[derive(Debug, Default)]
pub struct Strategies<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: StrategyRequest,
}

static ACTIVE_STRATEGIES: Lazy<Arc<RwLock<Option<HashMap<i32, strategies::Model>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

static STRATEGIES_EVALUATION_LOOP_ABORT_HANDLE: Lazy<Arc<RwLock<Option<AbortHandle>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Strategies {
    pub fn new(model: StrategyRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: StrategyRequest {
                ..Default::default()
            },
        }
    }

    pub fn into_model(strategy: StrategyRequest) -> Model {
        Model {
            id: strategy.id.unwrap_or_default(),
            name: strategy.name.unwrap_or_default(),
            is_active: strategy.is_active.unwrap_or_default(),
            can_trade: strategy.can_trade.unwrap_or_default(),
            description: Some(strategy.description.unwrap_or_default()),
            last_execution: Some(strategy.last_execution.unwrap_or_default()),
            cooldown: Some(strategy.cooldown.unwrap_or_default()),
        }
    }

    pub fn into_request(mut self, model: Model) -> Self {
        let strategy_request = StrategyRequest {
            id: Some(model.id),
            name: Some(model.name),
            is_active: Some(model.is_active),
            can_trade: Some(model.can_trade),
            description: model.description,
            last_execution: model.last_execution,
            cooldown: model.cooldown,
        };

        self.model = strategy_request;
        self
    }

    pub async fn set_active_strategies(strategies: Option<Vec<Model>>) -> Option<Vec<Model>> {
        let mut active_strategies = ACTIVE_STRATEGIES.write().await;

        let Some(strategies) = strategies else {
            *active_strategies = None;
            return None;
        };

        let mut active_strategies_map: HashMap<i32, Model> = HashMap::new();

        for strategy in strategies.iter() {
            active_strategies_map.insert(strategy.id, strategy.clone());
        }

        *active_strategies = Some(active_strategies_map);

        Some(strategies)
    }

    pub async fn set_active_strategy(strategy: Model) -> Model {
        let mut active_strategies = ACTIVE_STRATEGIES.write().await;

        // early return if active strategies is none
        let Some(strategies_map) = active_strategies.as_mut() else {
            return strategy;
        };

        // updates strategy if it's active or removes it if it isn't
        if strategy.is_active {
            strategies_map.insert(strategy.id, strategy.clone());
        } else {
            strategies_map.remove(&strategy.id);
        }

        strategy
    }

    pub async fn get_active_strategies() -> Option<HashMap<i32, Model>> {
        let active_strategies = ACTIVE_STRATEGIES.read().await;

        active_strategies.clone()
    }

    pub async fn get_active_strategy(key: &i32) -> Option<Model> {
        let active_strategies = ACTIVE_STRATEGIES.read().await;

        let Some(strategies_map) = active_strategies.as_ref() else {
            return None;
        };

        strategies_map.get(key).cloned()
    }

    pub async fn start_active_strategies() -> Result<(), Response> {
        if Self::get_active_strategies()
            .await
            .is_none_or(|active_strategies| active_strategies.is_empty())
        {
            let mut strategies_request = Self::default();
            strategies_request.model.is_active = Some(true);
            let active_strategies = strategies_request.next_phase().select_strategies().await?;
            Self::set_active_strategies(Some(active_strategies)).await;
        }

        Ok(())
    }

    pub async fn stop_active_strategies() {
        Self::set_active_strategies(None).await;
    }

    pub async fn start_strategies_evaluation_loop() {
        let mut stored_abort_handle = STRATEGIES_EVALUATION_LOOP_ABORT_HANDLE.write().await;

        if stored_abort_handle
            .as_ref()
            .is_some_and(|handle| !handle.is_finished())
        {
            return;
        }

        let senders = Senders::get_active_senders().await;
        let mut tickers_receiver = senders.event_sender.subscribe();

        let abort_handle = tokio::spawn(async move {
            while let Ok(ticker) = tickers_receiver.recv().await {
                tokio::spawn(async move {
                    let start = Instant::now();

                    let Some(ticker) = Tickers::get_ticker(ticker.symbol).await else {
                        return;
                    };

                    // get strategies associated to ticker
                    let subscribed_indicators = Indicators::get_subscribed_indicators()
                        .await
                        .and_then(|m| m.get(&ticker.symbol.to_ascii_uppercase()).cloned())
                        .unwrap_or_default()
                        .into_iter()
                        .collect::<Vec<i32>>();

                    for strategy_id in subscribed_indicators {
                        if let Some(strategy_overview) =
                            StrategiesOverview::get_memory_strategy_overview(
                                &strategy_id,
                                ticker.symbol.clone(),
                            )
                            .await
                        {
                            Self::evalute_active_strategies(strategy_overview).await
                        }
                    }

                    let now = Local::now().naive_local();
                    Metrics::set_active_metric(MetricType::All, start.elapsed(), now).await;
                });
            }
        })
        .abort_handle();

        *stored_abort_handle = Some(abort_handle);
    }

    pub async fn stop_strategies_evaluation_loop() {
        let mut stored_abort_handle = STRATEGIES_EVALUATION_LOOP_ABORT_HANDLE.write().await;

        if let Some(handle) = stored_abort_handle.as_ref() {
            handle.abort();
        }

        *stored_abort_handle = None;
    }

    pub async fn evalute_active_strategies(strategy_overview: StrategyOverview) {
        let start = Instant::now();

        let Ok(order) = Self::evaluate_custom_logic(&strategy_overview) else {
            return;
        };

        if strategy_overview.strategy.can_trade {}

        let order = match order.insert_order().await {
            Err(err) => {
                dbg!(eprintln!("{}", err.message));
                return;
            }
            Ok(val) => val,
        };

        // update base asset
        let (base_asset_request, base_asset_ledger) =
            Self::build_asset_ledger_request(&strategy_overview, &order, true);

        if let Err(err) = Assets::new(base_asset_request)
            .update_asset_value(
                order.base_asset_amount,
                false,
                strategy_overview.action.is_sell,
            )
            .await
        {
            dbg!(eprintln!("{}", err.message));
            return;
        };

        if let Err(err) = Ledgers::new(base_asset_ledger).insert_ledger().await {
            dbg!(eprintln!("{}", err.message));
            return;
        };

        // update quote asset
        let (quote_asset_request, quote_asset_ledger) =
            Self::build_asset_ledger_request(&strategy_overview, &order, false);

        if let Err(err) = Assets::new(quote_asset_request)
            .update_asset_value(
                order.quote_asset_amount,
                false,
                !strategy_overview.action.is_sell,
            )
            .await
        {
            dbg!(eprintln!("{}", err.message));
            return;
        };

        if let Err(err) = Ledgers::new(quote_asset_ledger).insert_ledger().await {
            dbg!(eprintln!("{}", err.message));
            return;
        };

        // update strategy last execution datetime
        let now = Local::now().naive_local();

        let mut strategy_request = Self::default().into_request(strategy_overview.strategy);
        strategy_request.model.last_execution = Some(now);

        if let Err(err) = strategy_request.update_strategy().await {
            dbg!(eprintln!("{}", err.message));
            return;
        };

        // update speed metrics
        let now = Local::now().naive_local();
        Metrics::set_active_metric(MetricType::Completed, start.elapsed(), now).await;

        // dbg!(format!(
        //     "Successful evaluation completed at {} -- duration of {:?}",
        //     now,
        //     start.elapsed()
        // ));
    }

    pub fn evaluate_custom_logic(strategy_overview: &StrategyOverview) -> Result<Orders, String> {
        // evaluate cooldown has passed
        if !Strategies::default().evaluate_cooldown(
            strategy_overview.strategy.last_execution,
            strategy_overview.strategy.cooldown,
        ) {
            return Err(format!("Cooldown is active"));
        }

        // evalute indicator
        match Indicators::evalute_active_indicators(
            &strategy_overview.ticker,
            &strategy_overview.indicator,
            &strategy_overview.pair,
        ) {
            Err(err) => return Err(err),
            Ok(false) => return Err(format!("Indicator not met")),
            Ok(true) => (),
        };

        // evaluate and modify action
        let action = match Actions::evaluate_active_actions(
            strategy_overview.action.clone(),
            &strategy_overview.pair,
            &strategy_overview.base_asset,
            &strategy_overview.quote_asset,
            &strategy_overview.ticker,
        ) {
            Err(err) => return Err(err),
            Ok(val) => val,
        };

        // create order
        let order = Orders::default()
            .from_strategy(&strategy_overview.strategy)
            .from_action(&action)
            .from_pair(&strategy_overview.pair)
            .from_ticker(&strategy_overview.ticker)
            .from_status(2);

        Ok(order)
    }

    pub fn build_asset_ledger_request(
        strategy_overview: &StrategyOverview,
        order: &orders::Model,
        is_base: bool,
    ) -> (AssetRequest, LedgerRequest) {
        let asset = match is_base {
            true => &strategy_overview.base_asset,
            false => &strategy_overview.quote_asset,
        };

        let value = match is_base {
            true => &order.base_asset_amount,
            false => &order.quote_asset_amount,
        };

        let previous_balance = match is_base {
            true => &strategy_overview.base_asset.free,
            false => &strategy_overview.quote_asset.free,
        };

        let asset_request =
            AssetRequest::from_model(asset).aggregate_values(*value, false, is_base);

        let asset_ledger = LedgerRequest::from_asset(asset)
            .from_order(&order)
            .update_values(
                false,
                *value,
                *previous_balance,
                asset_request.free.unwrap_or_default(),
            );

        (asset_request, asset_ledger)
    }
}

impl<Phase> Strategies<Phase> {
    pub fn next_phase<Next>(self) -> Strategies<Next> {
        Strategies {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Strategies<Types> {
    pub async fn insert_strategy(self) -> Result<Model, Response> {
        let strategy = self.insert_strategy_core().await?;
        let memory_strategy = Self::set_active_strategy(strategy).await;

        Ok(memory_strategy)
    }

    pub async fn select_strategy(self) -> Result<Option<Model>, Response> {
        let memory_strategy = Self::get_active_strategy(&self.model.id.unwrap_or_default()).await;

        if memory_strategy.is_some() {
            return Ok(memory_strategy);
        }

        self.select_strategy_core().await
    }

    pub async fn select_strategies(self) -> Result<Vec<Model>, Response> {
        self.select_strategies_core().await
    }

    pub async fn update_strategy(self) -> Result<Model, Response> {
        let strategy = self.update_strategy_core().await?;

        let memory_strategy = Self::set_active_strategy(strategy).await;

        Ok(memory_strategy)
    }

    pub async fn delete_strategy(self) -> Result<u64, Response> {
        let mut strategy = Self::into_model(self.model.clone());
        strategy.is_active = false;

        Self::set_active_strategy(strategy).await;

        self.delete_strategy_core().await
    }

    pub fn evaluate_cooldown(
        self,
        last_exec: Option<NaiveDateTime>,
        cooldown: Option<i32>,
    ) -> bool {
        self.evaluate_cooldown_core(last_exec, cooldown)
    }
}

#[cfg(test)]
mod fn_evaluate_custom_logic {
    use models::{
        entities::{actions, assets, indicators, pairs},
        structs::Ticker,
    };
    use sea_orm::prelude::Decimal;

    use super::*;

    fn make_strategy_overview() -> StrategyOverview {
        StrategyOverview {
            strategy: strategies::Model {
                id: 1,
                name: "Scalping".to_string(),
                is_active: true,
                can_trade: true,
                last_execution: None,
                cooldown: Some(1),
                ..Default::default()
            },
            indicator: indicators::Model {
                id: 1,
                strategy_id: 1,
                is_active: true,
                nick: "lst".to_string(),      // last_price
                direction: "gte".to_string(), // >=
                value: Decimal::ONE,          // 1
                ..Default::default()
            },
            action: actions::Model {
                id: 1,
                strategy_id: 1,
                is_active: true,
                is_sell: false,
                is_quote_asset: false,
                is_percentage: false,
                value: Decimal::ONE, // 1 base asset
                pair_id: 1,
            },
            pair: pairs::Model {
                id: 1,
                base_asset_id: 1,
                quote_asset_id: 2,
                symbol: "BTCUSDT".to_string(),
                update_date: chrono::Local::now().naive_local(),
                all_time_high_price: Decimal::new(100000, 0),
                all_time_high_date: chrono::Local::now().naive_local(),
                percent_from_all_time_high: Decimal::new(10, 0),
                lot_size_step_size: Decimal::ONE,
                notional_min_notional: Decimal::ONE,
                ..Default::default()
            },
            base_asset: assets::Model {
                id: 1,
                name: "BTC".to_string(),
                ticker: "BTC".to_string(),
                free: Decimal::new(10, 0),
                locked: Decimal::ZERO,
            },
            quote_asset: assets::Model {
                id: 2,
                name: "USDT".to_string(),
                ticker: "USDT".to_string(),
                free: Decimal::new(100000, 0),
                locked: Decimal::ZERO,
            },
            ticker: Ticker {
                symbol: "BTCUSDT".to_string(),
                last_price: Decimal::new(20000, 0),
                ..Default::default()
            },
        }
    }

    #[test]
    fn evaluate_custom_logic_cases() {
        let base_case = make_strategy_overview();

        let cases = vec![
            (
                "err_cooldown_active",
                {
                    let mut s = base_case.clone();
                    s.strategy.last_execution = Some(chrono::Local::now().naive_local());
                    s
                },
                false,
            ),
            (
                "err_indicator_not_met",
                {
                    let mut s = base_case.clone();
                    s.indicator.direction = "gt".to_string(); // exige last_price > 1
                    s.indicator.value = Decimal::new(30000, 0); // 30k
                    s
                },
                false,
            ),
            (
                "ok_valid_case",
                {
                    let s = base_case.clone();
                    s
                },
                true,
            ),
        ];

        for (name, overview, should_pass) in cases {
            let result = Strategies::evaluate_custom_logic(&overview);

            assert_eq!(
                result.is_ok(),
                should_pass,
                "case `{}` failed: expected {}, got {:?}",
                name,
                should_pass,
                result
            );
        }
    }
}

#[cfg(test)]
mod fn_build_asset_ledger_request {
    use models::entities::assets;
    use sea_orm::prelude::Decimal;

    use super::*;

    fn make_strategy_overview() -> StrategyOverview {
        StrategyOverview {
            base_asset: assets::Model {
                id: 1,
                name: "BTC".to_string(),
                ticker: "BTC".to_string(),
                free: Decimal::new(10, 0),
                locked: Decimal::ZERO,
            },
            quote_asset: assets::Model {
                id: 2,
                name: "USDT".to_string(),
                ticker: "USDT".to_string(),
                free: Decimal::new(1000, 0),
                locked: Decimal::ZERO,
            },
            ..Default::default()
        }
    }

    fn make_order() -> orders::Model {
        orders::Model {
            id: 1,
            status_id: 1,
            creation_date: chrono::Local::now().naive_local(),
            update_date: chrono::Local::now().naive_local(),
            is_sell: false,
            strategy_id: 1,
            base_asset_id: 1,
            base_asset_amount: Decimal::new(2, 0), // 2 BTC
            quote_asset_id: 2,
            quote_asset_amount: Decimal::new(40000, 0), // 40,000 USDT
            price_entry: Decimal::new(20000, 0),
            price_target: Decimal::new(20000, 0),
            price_abort: Decimal::ZERO,
        }
    }

    #[test]
    fn build_asset_ledger_request_cases() {
        let overview = make_strategy_overview();
        let order = make_order();

        let cases = vec![
            ("ok_base_asset", true, overview.clone(), order.clone()),
            ("ok_quote_asset", false, overview.clone(), order.clone()),
        ];

        for (name, is_base, overview, order) in cases {
            let (asset_req, ledger_req) =
                Strategies::build_asset_ledger_request(&overview, &order, is_base);

            if is_base {
                assert_eq!(
                    asset_req.id,
                    Some(overview.base_asset.id),
                    "case `{}` failed: expected base_asset id",
                    name
                );
                assert_eq!(
                    ledger_req.asset_id,
                    Some(overview.base_asset.id),
                    "case `{}` failed: expected ledger to use base_asset",
                    name
                );
            } else {
                assert_eq!(
                    asset_req.id,
                    Some(overview.quote_asset.id),
                    "case `{}` failed: expected quote_asset id",
                    name
                );
                assert_eq!(
                    ledger_req.asset_id,
                    Some(overview.quote_asset.id),
                    "case `{}` failed: expected ledger to use quote_asset",
                    name
                );
            }
        }
    }
}
