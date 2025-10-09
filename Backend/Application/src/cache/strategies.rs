use std::{collections::HashMap, sync::Arc, time::Instant};

use chrono::Local;
use models::{
    entities::strategies::{self, Model},
    enums::MetricType,
    structs::StrategyOverview,
};
use once_cell::sync::Lazy;
use tokio::{sync::RwLock, task::AbortHandle};

use crate::{
    handler::{
        Assets, Binance, Ledgers, Metrics, Senders, Strategies, StrategiesOverview,
        SubscribedIndicators, Tickers,
    },
    utils::{Cache, Response},
};

static ACTIVE_STRATEGIES: Lazy<Arc<RwLock<Option<HashMap<i32, strategies::Model>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

static STRATEGIES_EVALUATION_LOOP_ABORT_HANDLE: Lazy<Arc<RwLock<Option<AbortHandle>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Strategies<Cache> {
    pub async fn set_active_strategies_cache(strategies: Option<Vec<Model>>) -> Option<Vec<Model>> {
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

    pub async fn set_active_strategy_cache(strategy: Model) -> Model {
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

    pub async fn get_active_strategies_cache() -> Option<HashMap<i32, Model>> {
        let active_strategies = ACTIVE_STRATEGIES.read().await;

        active_strategies.clone()
    }

    pub async fn get_active_strategy_cache(key: &i32) -> Option<Model> {
        let active_strategies = ACTIVE_STRATEGIES.read().await;

        let Some(strategies_map) = active_strategies.as_ref() else {
            return None;
        };

        strategies_map.get(key).cloned()
    }

    pub async fn start_active_strategies_cache() -> Result<(), Response> {
        if Strategies::get_active_strategies_cache()
            .await
            .is_none_or(|active_strategies| active_strategies.is_empty())
        {
            let mut strategies_request = Strategies::default();
            strategies_request.model.is_active = Some(true);

            let active_strategies = strategies_request.next_phase().select_strategies().await?;
            Strategies::set_active_strategies_cache(Some(active_strategies)).await;
        }

        Ok(())
    }

    pub async fn stop_active_strategies_cache() {
        Strategies::set_active_strategies_cache(None).await;
    }

    pub async fn start_strategies_evaluation_loop_cache() {
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
                    let subscribed_indicators = SubscribedIndicators::default()
                        .select_subscribed_indicators()
                        .await
                        .and_then(|m| m.get(&ticker.symbol.to_ascii_uppercase()).cloned())
                        .unwrap_or_default()
                        .into_iter()
                        .collect::<Vec<i32>>();

                    for strategy_id in subscribed_indicators {
                        if let Some(strategy_overview) =
                            StrategiesOverview::get_active_strategy_overview(
                                &strategy_id,
                                ticker.symbol.clone(),
                            )
                            .await
                        {
                            Strategies::<Cache>::evalute_active_strategies_cache(strategy_overview)
                                .await;
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

    pub async fn stop_strategies_evaluation_loop_cache() {
        let mut stored_abort_handle = STRATEGIES_EVALUATION_LOOP_ABORT_HANDLE.write().await;

        if let Some(handle) = stored_abort_handle.as_ref() {
            handle.abort();
        }

        *stored_abort_handle = None;
    }

    pub async fn evalute_active_strategies_cache(strategy_overview: StrategyOverview) {
        let start = Instant::now();

        let Ok(mut order) = StrategiesOverview::evaluate_strategy_overview(&strategy_overview)
        else {
            return;
        };

        // check assets aren't locked trading
        if Assets::get_posting_asset(&strategy_overview.base_asset.id)
            .await
            .is_some_and(|val| val)
        {
            return;
        }

        if Assets::get_posting_asset(&strategy_overview.quote_asset.id)
            .await
            .is_some_and(|val| val)
        {
            return;
        };

        // lock assets for trades
        Assets::set_posting_asset(strategy_overview.base_asset.id, true, false).await;
        Assets::set_posting_asset(strategy_overview.quote_asset.id, true, false).await;

        if strategy_overview.strategy.can_trade {
            if Binance::default()
                .post_new_order(strategy_overview.ticker.symbol.clone(), &mut order.model)
                .await
                .is_err()
            {
                Strategies::update_strategy_last_dates(&strategy_overview.strategy, true).await;
                return;
            }
        }

        let order = match order.insert_order().await {
            Err(err) => {
                dbg!(eprintln!("{}", err.message));
                Strategies::update_strategy_last_dates(&strategy_overview.strategy, true).await;
                return;
            }
            Ok(val) => val,
        };

        // update base asset
        let (base_asset_request, base_asset_ledger) =
            StrategiesOverview::build_asset_ledger_request(&strategy_overview, &order, true);

        if let Err(err) = Assets::new(base_asset_request)
            .update_asset_value(
                order.base_asset_amount,
                false,
                strategy_overview.action.is_sell,
            )
            .await
        {
            dbg!(eprintln!("{}", err.message));
            Strategies::update_strategy_last_dates(&strategy_overview.strategy, true).await;
            return;
        };

        if let Err(err) = Ledgers::new(base_asset_ledger).insert_ledger().await {
            dbg!(eprintln!("{}", err.message));
            Strategies::update_strategy_last_dates(&strategy_overview.strategy, true).await;
            return;
        };

        // update quote asset
        let (quote_asset_request, quote_asset_ledger) =
            StrategiesOverview::build_asset_ledger_request(&strategy_overview, &order, false);

        if let Err(err) = Assets::new(quote_asset_request)
            .update_asset_value(
                order.quote_asset_amount,
                false,
                !strategy_overview.action.is_sell,
            )
            .await
        {
            dbg!(eprintln!("{}", err.message));
            Strategies::update_strategy_last_dates(&strategy_overview.strategy, true).await;
            return;
        };

        if let Err(err) = Ledgers::new(quote_asset_ledger).insert_ledger().await {
            dbg!(eprintln!("{}", err.message));
            Strategies::update_strategy_last_dates(&strategy_overview.strategy, true).await;
            return;
        };

        // update strategy last execution datetime
        Strategies::update_strategy_last_dates(&strategy_overview.strategy, false).await;

        // unlock assets for new trades
        Assets::set_posting_asset(strategy_overview.base_asset.id, false, false).await;
        Assets::set_posting_asset(strategy_overview.quote_asset.id, false, false).await;

        // update speed metrics
        let now = Local::now().naive_local();
        Metrics::set_active_metric(MetricType::Completed, start.elapsed(), now).await;
    }

    async fn update_strategy_last_dates(strategy: &Model, is_error: bool) {
        let now = Local::now().naive_local();

        let mut strategy_request = Strategies::default().into_request(strategy.clone());

        if is_error {
            strategy_request.model.error_last_date = Some(now);
        } else {
            strategy_request.model.last_execution = Some(now);
        }

        if let Err(err) = strategy_request.update_strategy().await {
            dbg!(eprintln!("{}", err.message));
            return;
        };
    }
}
