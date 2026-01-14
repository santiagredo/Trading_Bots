use chrono::NaiveDateTime;
use models::{
    entities::strategies::Model,
    enums::{LifecycleState, TradingState},
    structs::{CacheStrategies, CacheStrategy, Environments, StrategyOverview},
};
use tokio_util::sync::CancellationToken;

use crate::{
    guard::StrategiesExecutionGuard,
    handler::{
        Binance, Senders, Strategies, StrategiesOverview, SubscribedIndicators, Tickers, DBC,
    },
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Strategies<Core> {
    /* ===========================
     * DB
     * ===========================
     */

    pub async fn insert_strategy_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .insert_strategy_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .insert_strategy_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_strategy_core(self) -> Result<Option<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_strategy_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_strategies_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_strategies_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn update_strategy_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .update_strategy_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .update_strategy_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn delete_strategy_core(self) -> Result<u64, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .delete_strategy_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .delete_strategy_data(&DBC::db(&env).await?)
            .await
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_strategies_core(self) -> Option<CacheStrategies> {
        Strategies::<Cache>::get_strategies_cache(self.environment).await
    }

    pub async fn get_strategy_core(self) -> Option<CacheStrategy> {
        Strategies::<Cache>::get_strategy_cache(self.environment, self.model.id.unwrap_or_default())
            .await
    }

    pub async fn get_strategies_state_core(self) -> LifecycleState {
        Strategies::<Cache>::get_strategies_state_cache(self.environment).await
    }

    /* ===========================
     * CACHE (WRITE)
     * ===========================
     */

    pub async fn upsert_strategy_core(self) -> Result<(), String> {
        let env = self.environment;
        let model = Strategies::into_model(self.model);

        Strategies::<Cache>::upsert_strategy_cache(env, model).await
    }

    pub async fn remove_strategy_core(self) -> Result<Option<CacheStrategy>, String> {
        let env = self.environment;
        let id = self.model.id.unwrap_or_default();

        Strategies::<Cache>::remove_strategy_cache(env, id).await
    }

    pub async fn set_strategy_error_core(self, error: Option<String>) -> Result<(), String> {
        Strategies::<Cache>::set_strategy_error_cache(
            self.environment,
            self.model.id.unwrap_or_default(),
            error,
        )
        .await
    }

    pub async fn set_strategy_state_core(self, state: TradingState) -> Result<(), String> {
        Strategies::<Cache>::set_strategy_state_cache(
            self.environment,
            self.model.id.unwrap_or_default(),
            state,
        )
        .await
    }

    /* ===========================
     * START ACTIVE STRATEGIES
     * ===========================
     */

    pub async fn start_strategies_core(self, token: &CancellationToken) -> Result<(), Response> {
        let env = self.environment;

        // STARTING
        Strategies::<Cache>::set_status_cache(env, LifecycleState::Starting)
            .await
            .map_err(|e| Response {
                code: 500,
                message: e,
            })?;

        // Load from DB
        let models = match Strategies::default()
            .with_env(env)
            .select_strategies()
            .await
        {
            Ok(m) => m.into_iter().filter(|s| s.is_active).collect(),
            Err(err) => {
                let _ = Strategies::<Cache>::reset_strategies_cache(env).await;
                return Err(Response {
                    code: 500,
                    message: err.message,
                });
            }
        };

        // RUNNING
        Strategies::<Cache>::set_status_cache(env, LifecycleState::Running)
            .await
            .map_err(|e| Response {
                code: 500,
                message: e,
            })?;

        // Set cache
        Strategies::<Cache>::set_strategies_cache(env, models)
            .await
            .map_err(|e| Response {
                code: 500,
                message: e,
            })?;

        // Runtime
        let senders = Senders::get_senders().await;
        let mut tickers_receiver = senders.event_sender.subscribe();

        let cancellation_token = token.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        break;
                    }

                    msg = tickers_receiver.recv() => {
                        let Ok(ticker_evt) = msg else {
                            continue;
                        };

                        let environment = env;

                        let Some(ticker) = Tickers::get_ticker(ticker_evt.symbol).await else {
                            continue;
                        };

                        let subscribed = SubscribedIndicators::new(environment)
                            .get_subscribed_indicators()
                            .await
                            .and_then(|m| m.get(&ticker.symbol).cloned())
                            .unwrap_or_default();

                        for strategy_id in subscribed {
                            if let Some(overview) = StrategiesOverview::get_strategy_overview(
                                environment,
                                &strategy_id,
                                ticker.symbol.clone(),
                            )
                            .await
                            {
                                Self::evaluate_strategy(overview, environment).await;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /* ===========================
     * STOP ACTIVE STRATEGIES
     * ===========================
     */

    pub async fn stop_strategies_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STOPPING
        Strategies::<Cache>::set_status_cache(env, LifecycleState::Stopping)
            .await
            .map_err(|e| Response {
                code: 500,
                message: e,
            })?;

        // Remove cache
        Strategies::<Cache>::remove_strategies_cache(env)
            .await
            .map_err(|e| Response {
                code: 500,
                message: e,
            })?;

        // OFF
        Strategies::<Cache>::set_status_cache(env, LifecycleState::Off)
            .await
            .map_err(|e| Response {
                code: 500,
                message: e,
            })?;

        Ok(())
    }

    /* ===========================
     * Runtime evaluation
     * ===========================
     */

    async fn evaluate_strategy(strategy_overview: StrategyOverview, environment: Environments) {
        // READY -> RUNNING
        let mut guard = match StrategiesExecutionGuard::new(
            environment,
            Strategies::default()
                .into_request(strategy_overview.strategy.clone())
                .model,
        )
        .await
        {
            Err(err) => {
                dbg!(err);
                return;
            }
            Ok(val) => val,
        };

        let mut order = match StrategiesOverview::evaluate_strategy_overview(&strategy_overview) {
            Ok(o) => o,
            Err(err) => {
                // RUNNING -> READY
                let _ = guard.err(err).await;

                return;
            }
        };

        // RUNNING -> TRADING
        if let Err(err) = guard.lock().await {
            // RUNNING -> READY
            let _ = guard.err(err).await;

            return;
        }

        if strategy_overview.strategy.can_trade {
            let (api_key, secret_pass) = match Binance::default()
                .with_env(environment)
                .resolve_binance_credentials()
                .await
            {
                Ok(val) => val,
                Err(err) => {
                    // TRADING -> READY
                    let _ = guard.ext_err(err.message).await;

                    return;
                }
            };

            if let Err(err) = Binance::default()
                .post_new_order(
                    strategy_overview.ticker.symbol.clone(),
                    &mut order.model,
                    api_key,
                    secret_pass,
                )
                .await
            {
                // TRADING -> READY
                let _ = guard.ext_err(err.message).await;

                return;
            }
        }

        // TRADING -> SAVING
        let _ = guard.saving().await;

        match order.insert_order().await {
            Err(err) => {
                let _ = guard.err(err.message).await;

                return;
            }
            Ok(order) => {
                let senders = Senders::get_senders().await;
                let _ = senders.order_sender.send((environment, order));
            }
        };

        // DROP: SAVING -> READY
        guard.ok();
    }

    pub async fn reset_strategies_core(self) -> Result<(), Response> {
        Strategies::<Cache>::reset_strategies_cache(self.environment)
            .await
            .map_err(Response::server_error)
    }

    /* ===========================
     * Misc
     * ===========================
     */

    pub fn evaluate_cooldown_core(
        self,
        last_exec: Option<NaiveDateTime>,
        cooldown: Option<i32>,
    ) -> bool {
        self.next_phase()
            .evaluate_cooldown_logic(last_exec, cooldown)
    }
}
