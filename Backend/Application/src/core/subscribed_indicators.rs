use std::collections::{HashMap, HashSet};

use models::{entities::indicators::Model, enums::LifecycleState, structs::Environments};

use crate::{
    handler::{Indicators, SubscribedIndicators},
    utils::{Cache, Core, Response},
};

impl SubscribedIndicators<Core> {
    /* ===========================
     * CACHE (WRITE)
     * ===========================
     */

    pub async fn upsert_subscribed_indicator_core(self, indicator: Model) -> Result<(), Response> {
        let env = self.environment;

        SubscribedIndicators::<Cache>::upsert_subscribed_indicator_cache(env, indicator)
            .await
            .map_err(|err| Response {
                code: 500,
                message: err,
            })
    }

    pub async fn remove_subscribed_indicator_core(
        self,
        indicator: Model,
    ) -> Result<Option<Model>, Response> {
        let env = self.environment;

        SubscribedIndicators::<Cache>::remove_subscribed_indicator_cache(env, indicator.clone())
            .await
            .map(|_| Some(indicator))
            .map_err(|err| Response {
                code: 500,
                message: err,
            })
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_subscribed_indicator_core(self, key: String) -> Option<HashSet<i32>> {
        SubscribedIndicators::<Cache>::get_subscribed_indicator_cache(self.environment, &key).await
    }

    pub async fn get_subscribed_indicators_core(self) -> Option<HashMap<String, HashSet<i32>>> {
        SubscribedIndicators::<Cache>::get_subscribed_indicators_cache(self.environment).await
    }

    pub async fn get_subscribed_indicators_state_core(self) -> LifecycleState {
        SubscribedIndicators::<Cache>::get_subscribed_indicators_state_cache(self.environment).await
    }

    /* ===========================
     * LIFECYCLE
     * ===========================
     */

    pub async fn start_subscribed_indicators_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STARTING
        if let Err(err) =
            SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Starting).await
        {
            return Err(Response {
                code: 500,
                message: err,
            });
        };

        let mut indicators_request = Indicators::default();
        indicators_request.environment = env;

        let active_indicators = indicators_request
            .get_indicators()
            .await
            .unwrap_or_default();

        let mut indicators_map: HashMap<String, HashSet<i32>> = HashMap::new();
        for (_, indicator) in active_indicators {
            indicators_map
                .entry(indicator.symbol.clone())
                .and_modify(|val| {
                    val.insert(indicator.strategy_id);
                })
                .or_insert_with(HashSet::new)
                .insert(indicator.strategy_id);
        }

        // RUNNING
        if let Err(err) =
            SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Running).await
        {
            return Err(Response {
                code: 500,
                message: err,
            });
        };

        if let Err(err) =
            SubscribedIndicators::<Cache>::set_subscribed_indicators_cache(env, indicators_map)
                .await
        {
            return Err(Response {
                code: 500,
                message: err,
            });
        };

        Ok(())
    }

    pub async fn stop_subscribed_indicators_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STOPPING
        if let Err(err) =
            SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Stopping).await
        {
            return Err(Response {
                code: 500,
                message: err,
            });
        };

        if let Err(err) =
            SubscribedIndicators::<Cache>::remove_all_subscribed_indicators_cache(env).await
        {
            return Err(Response {
                code: 500,
                message: err,
            });
        };

        // OFF
        if let Err(err) =
            SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Off).await
        {
            return Err(Response {
                code: 500,
                message: err,
            });
        };

        Ok(())
    }

    pub async fn reset_subscribed_indicators_core(self) -> Result<(), Response> {
        SubscribedIndicators::<Cache>::reset_subscribed_indicators_cache(self.environment)
            .await
            .map_err(Response::server_error)
    }

    pub async fn get_all_unique_symbols_core() -> Result<Vec<String>, String> {
        let mut symbols = std::collections::HashSet::new();

        for env in [Environments::DEV, Environments::PROD] {
            if let Some(indicators) = SubscribedIndicators::new(env)
                .get_subscribed_indicators()
                .await
            {
                symbols.extend(indicators.keys().cloned());
            }
        }

        Ok(symbols.into_iter().collect())
    }
}
