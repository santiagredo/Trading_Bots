use std::{collections::HashMap, marker::PhantomData, sync::Arc, time::Instant};

use chrono::{Local, NaiveDateTime};
use models::{
    entities::strategies::{self, Model},
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

        let Ok(mut order) = Self::evaluate_custom_logic(&strategy_overview) else {
            return;
        };
        order.model.status_id = Some(2);

        if strategy_overview.strategy.can_trade {}

        let order = match order.insert_order().await {
            Err(err) => {
                println!("{}", err.message);
                return;
            }
            Ok(val) => val,
        };

        // update base asset
        let base_asset_request = AssetRequest::from_model(&strategy_overview.base_asset)
            .update_values(false, order.base_asset_amount);

        let base_asset_model = match Assets::new(base_asset_request)
            .update_asset_value(
                order.base_asset_amount,
                false,
                strategy_overview.action.is_sell,
            )
            .await
        {
            Err(err) => {
                println!("{}", err.message);
                return;
            }
            Ok(val) => val,
        };

        let base_asset_ledger = LedgerRequest::from_asset(&strategy_overview.base_asset)
            .from_order(&order)
            .update_values(
                false,
                order.base_asset_amount,
                strategy_overview.base_asset.free,
                base_asset_model.free,
            );

        if let Err(err) = Ledgers::new(base_asset_ledger).insert_ledger().await {
            println!("{}", err.message);
            return;
        };

        // update quote asset
        let quote_asset_request = AssetRequest::from_model(&strategy_overview.quote_asset)
            .update_values(false, order.quote_asset_amount);

        let quote_asset_model = match Assets::new(quote_asset_request)
            .update_asset_value(
                order.quote_asset_amount,
                false,
                !strategy_overview.action.is_sell,
            )
            .await
        {
            Err(err) => {
                println!("{}", err.message);
                return;
            }
            Ok(val) => val,
        };

        let quote_asset_ledger = LedgerRequest::from_asset(&strategy_overview.quote_asset)
            .from_order(&order)
            .update_values(
                false,
                order.quote_asset_amount,
                strategy_overview.quote_asset.free,
                quote_asset_model.free,
            );

        if let Err(err) = Ledgers::new(quote_asset_ledger).insert_ledger().await {
            println!("{}", err.message);
            return;
        };

        // update strategy last execution datetime
        let now = Local::now().naive_local();

        let mut strategy_request = Self::default().into_request(strategy_overview.strategy);
        strategy_request.model.last_execution = Some(now);

        match strategy_request.update_strategy().await {
            Err(err) => {
                println!("{}", err.message);
                return;
            }
            Ok(val) => val,
        };

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
            .from_ticker(&strategy_overview.ticker);

        Ok(order)
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
