use std::collections::HashMap;

use models::{
    entities::{
        indicators::{self, Model},
        pairs,
    },
    enums::LifecycleState,
    structs::Ticker,
};

use crate::{
    handler::{Indicators, Strategies, DBC},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Indicators<Core> {
    /* ===========================
     * DB
     * ===========================
     */

    pub async fn insert_indicator_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .insert_indicator_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .insert_indicator_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_indicator_core(self) -> Result<Option<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_indicator_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_indicators_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_indicators_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn update_indicator_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .update_indicator_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .update_indicator_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn delete_indicator_core(self) -> Result<u64, Response> {
        let env = self.environment;

        self.next_phase::<Logic>()
            .delete_indicator_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .delete_indicator_data(&DBC::db(&env).await?)
            .await
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_indicators_core(self) -> Option<HashMap<i32, Model>> {
        Indicators::<Cache>::get_indicators_cache(self.environment)
            .await
            .map(|c| c.models)
    }

    pub async fn get_indicator_core(self) -> Option<Model> {
        let id = self.model.id.or(self.model.strategy_id).unwrap_or_default();

        Indicators::<Cache>::get_indicator_cache(self.environment, id).await
    }

    pub async fn get_indicators_state_core(self) -> LifecycleState {
        Indicators::<Cache>::get_cache_state(self.environment).await
    }

    /* ===========================
     * CACHE (WRITE)
     * ===========================
     */

    pub async fn upsert_indicator_core(self) -> Result<(), Response> {
        let env = self.environment;
        let model = Indicators::into_model(self.model);

        Indicators::<Cache>::upsert_indicator_cache(env, model)
            .await
            .map_err(|err| Response {
                code: 500,
                message: err,
            })
    }

    pub async fn remove_indicator_core(self) -> Result<Option<Model>, Response> {
        let env = self.environment;
        let id = self.model.strategy_id.unwrap_or_default();

        Indicators::<Cache>::remove_indicator_cache(env, id)
            .await
            .map_err(|err| Response {
                code: 500,
                message: err,
            })
    }

    /* ===========================
     * START ACTIVE INDICATORS
     * ===========================
     */

    pub async fn start_indicators_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STARTING
        if let Err(err) = Indicators::<Cache>::set_status_cache(env, LifecycleState::Starting).await
        {
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Load active strategies
        let mut strategies_request = Strategies::default().with_env(env);
        strategies_request.model.is_active = Some(true);

        let active_strategies = strategies_request
            .get_strategies()
            .await
            .unwrap_or_default();

        // Load indicators from DB
        let mut indicators_request = Indicators::default().with_env(env);
        indicators_request.model.is_active = Some(true);

        let indicators = match indicators_request.select_indicators().await {
            Ok(i) => i
                .into_iter()
                .filter(|ind| active_strategies.models.contains_key(&ind.strategy_id))
                .collect::<Vec<_>>(),
            Err(err) => {
                let _ = Indicators::<Cache>::reset_indicators_cache(env).await;
                return Err(err);
            }
        };

        // RUNNING
        if let Err(err) = Indicators::<Cache>::set_status_cache(env, LifecycleState::Running).await
        {
            let _ = Indicators::<Cache>::reset_indicators_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Populate cache
        if let Err(err) = Indicators::<Cache>::set_indicators_cache(env, indicators).await {
            let _ = Indicators::<Cache>::reset_indicators_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    /* ===========================
     * STOP ACTIVE INDICATORS
     * ===========================
     */

    pub async fn stop_indicators_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STOPPING
        if let Err(err) = Indicators::<Cache>::set_status_cache(env, LifecycleState::Stopping).await
        {
            let _ = Indicators::<Cache>::reset_indicators_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // Remove indicators
        if let Err(err) = Indicators::<Cache>::remove_indicators_cache(env).await {
            let _ = Indicators::<Cache>::reset_indicators_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // OFF
        if let Err(err) = Indicators::<Cache>::set_status_cache(env, LifecycleState::Off).await {
            let _ = Indicators::<Cache>::reset_indicators_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    pub async fn reset_indicators_core(self) -> Result<(), Response> {
        Indicators::<Cache>::reset_indicators_cache(self.environment)
            .await
            .map_err(Response::server_error)
    }

    /* ===========================
     * MISC
     * ===========================
     */

    pub fn evaluate_indicator_core(
        self,
        indicator: &indicators::Model,
        ticker: &Ticker,
        pair: &pairs::Model,
    ) -> Result<bool, String> {
        self.next_phase::<Logic>()
            .evaluate_indicator_logic(indicator, ticker, pair)
    }
}
