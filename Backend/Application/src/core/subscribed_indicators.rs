use std::collections::{HashMap, HashSet};

use crate::{
    handler::{Indicators, SubscribedIndicators},
    utils::{EntityCache, Response},
};
use models::{enums::LifecycleState, structs::Environments};

impl<R> SubscribedIndicators<R>
where
    R: Send + Sync,
{
    /* ===========================
     * LIFECYCLE
     * ===========================
     */

    pub async fn start_subscribed_indicators(
        &self,
        environment: Environments,
    ) -> Result<(), Response> {
        // STARTING
        self.set_state(environment, LifecycleState::Starting)
            .await
            .map_err(Response::server_error)?;

        // LOAD ACTIVE INDICATORS
        let active_indicators = Indicators::blank()
            .get_all(environment)
            .await
            .unwrap_or_default();

        let mut indicators_map: HashMap<String, HashSet<i32>> = HashMap::new();

        for (_, indicator) in active_indicators.models {
            indicators_map
                .entry(indicator.symbol)
                .or_insert_with(HashSet::new)
                .insert(indicator.strategy_id);
        }

        let indicators_vec: Vec<(String, Vec<i32>)> = indicators_map
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        // RUNNING
        self.set_state(environment, LifecycleState::Running)
            .await
            .map_err(Response::server_error)?;

        self.set_all(environment, indicators_vec)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }

    pub async fn stop_subscribed_indicators(
        &self,
        environment: Environments,
    ) -> Result<(), Response> {
        // STOPPING
        self.set_state(environment, LifecycleState::Stopping)
            .await
            .map_err(Response::server_error)?;

        self.remove_all(environment)
            .await
            .map_err(Response::server_error)?;

        // OFF
        self.set_state(environment, LifecycleState::Off)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }

    pub async fn reset_subscribed_indicators(
        &self,
        environment: Environments,
    ) -> Result<(), Response> {
        self.reset(environment)
            .await
            .map_err(Response::server_error)
    }

    pub async fn get_all_unique_symbols(&self) -> Result<Vec<String>, String> {
        let mut symbols = std::collections::HashSet::new();

        for env in [Environments::DEV, Environments::PROD] {
            if let Some(indicators) = self.get_all(env).await {
                for key in indicators.models.keys() {
                    symbols.insert(key.clone());
                }
            }
        }

        Ok(symbols.into_iter().collect())
    }
}
