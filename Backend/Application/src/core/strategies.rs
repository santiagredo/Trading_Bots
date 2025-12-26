use std::collections::HashMap;

use chrono::NaiveDateTime;
use models::{
    entities::strategies::Model,
    structs::{CacheStrategy, Environments, StrategyOverview},
};

use crate::{
    guard::StrategiesExecutionGuard,
    handler::{
        Binance, Senders, Strategies, StrategiesOverview, SubscribedIndicators, Tickers, DBC,
    },
    utils::{handle_user_err, Cache, Data, Logic, Response},
};

impl<Core> Strategies<Core> {
    // db
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

    // cache
    pub async fn get_active_strategies_core(self) -> Option<HashMap<i32, CacheStrategy>> {
        let env = self.environment;
        Strategies::<Cache>::get_active_strategies_cache(&env).await
    }

    pub async fn get_active_strategy_core(self) -> Option<CacheStrategy> {
        let env = self.environment;
        let id = self.model.id.unwrap_or_default();

        Strategies::<Cache>::get_active_strategy_cache(&env, &id).await
    }

    pub async fn set_active_strategy_core(self, is_remove: bool, error: Option<String>) -> Model {
        let env = self.environment;
        let model = Strategies::into_model(self.model);

        Strategies::<Cache>::set_active_strategy_cache(&env, model, is_remove, error).await
    }

    pub async fn set_active_strategy_posting_core(self, is_posting: bool) -> Result<(), String> {
        let environment = self.environment;
        let strategy = self.model.id.unwrap_or_default();

        Strategies::<Cache>::set_active_strategy_posting_cache(environment, strategy, is_posting)
            .await
    }

    pub async fn stop_active_strategies_core(self) {
        let env = self.environment;

        Strategies::<Cache>::stop_active_strategies_cache(&env).await;
    }

    pub async fn start_active_strategies_core(self) -> Result<(), Response> {
        if !Strategies::<Cache>::get_active_strategies_status_cache(&self.environment).await {
            let mut strategies_request = Strategies::default().with_env(self.environment);
            strategies_request.model.is_active = Some(true);

            let active_strategies = strategies_request
                .select_strategies()
                .await
                .unwrap_or_default();

            Strategies::<Cache>::set_active_strategies_cache(&self.environment, active_strategies)
                .await;

            let senders = Senders::get_active_senders().await;
            let mut tickers_receiver = senders.event_sender.subscribe();

            let join_handle = tokio::spawn(async move {
                while let Ok(ticker) = tickers_receiver.recv().await {
                    tokio::spawn(async move {
                        let environment = self.environment;

                        let Some(ticker) = Tickers::get_ticker(ticker.symbol).await else {
                            return;
                        };

                        let subscribed_indicators_request =
                            SubscribedIndicators::new(self.environment);

                        // get strategies associated to ticker
                        let subscribed_indicators = subscribed_indicators_request
                            .get_active_subscribed_indicators()
                            .await
                            .and_then(|m| m.get(&ticker.symbol.to_ascii_uppercase()).cloned())
                            .unwrap_or_default()
                            .into_iter()
                            .collect::<Vec<i32>>();

                        for strategy_id in subscribed_indicators {
                            if let Some(strategy_overview) =
                                StrategiesOverview::get_active_strategy_overview(
                                    environment,
                                    &strategy_id,
                                    ticker.symbol.clone(),
                                )
                                .await
                            {
                                Self::evalute_active_strategies_core(
                                    strategy_overview,
                                    environment,
                                )
                                .await;
                            }
                        }
                    });
                }
            });

            Strategies::<Cache>::set_active_strategies_join_handle_cache(
                &self.environment,
                join_handle,
            )
            .await;
        }

        Ok(())
    }

    async fn evalute_active_strategies_core(
        strategy_overview: StrategyOverview,
        environment: Environments,
    ) {
        let mut execution_guard = StrategiesExecutionGuard::new(
            environment,
            Strategies::default()
                .into_request(strategy_overview.strategy.clone())
                .model,
        );

        execution_guard.err();

        let mut order = match StrategiesOverview::evaluate_strategy_overview(&strategy_overview) {
            Err(err) => {
                Strategies::default()
                    .into_request(strategy_overview.strategy)
                    .with_env(environment)
                    .set_active_strategy(false, Some(err))
                    .await;

                execution_guard.none();

                return;
            }
            Ok(val) => val,
        };

        if let Err(err) = execution_guard.lock().await {
            Strategies::default()
                .into_request(strategy_overview.strategy)
                .with_env(environment)
                .set_active_strategy(false, Some(err))
                .await;

            return;
        }

        if strategy_overview.strategy.can_trade {
            if Binance::default()
                .post_new_order(strategy_overview.ticker.symbol.clone(), &mut order.model)
                .await
                .is_err()
            {
                return;
            }
        }

        let order = match order.insert_order().await {
            Err(err) => {
                dbg!(eprintln!("{}", err.message));
                return;
            }
            Ok(val) => val,
        };

        let senders = Senders::get_active_senders().await;
        if let Err(err) = senders.order_sender.send((environment, order)) {
            dbg!(eprint!("Failed to send order command: {}", err.to_string()));
            return;
        }

        // update speed metrics
        execution_guard.ok();
    }

    // misc
    pub fn evaluate_cooldown_core(
        self,
        last_exec: Option<NaiveDateTime>,
        cooldown: Option<i32>,
    ) -> bool {
        self.next_phase()
            .evaluate_cooldown_logic(last_exec, cooldown)
    }
}
