use std::collections::HashMap;

use models::{
    entities::{
        indicators::{self, Model},
        pairs,
    },
    structs::Ticker,
};

use crate::{
    handler::{Indicators, Strategies, DBC},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Indicators<Core> {
    // db
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

    // cache
    pub async fn get_active_indicators_core(self) -> Option<HashMap<i32, Model>> {
        Indicators::<Cache>::get_active_indicators_cache(&self.environment).await
    }

    pub async fn get_active_indicator_core(self) -> Option<Model> {
        let env = self.environment;

        let id = if self.model.id.is_some() {
            self.model.id.unwrap_or_default()
        } else {
            self.model.strategy_id.unwrap_or_default()
        };

        Indicators::<Cache>::get_active_indicator_cache(&env, &id).await
    }

    pub async fn start_active_indicators_core(self) -> Result<(), Response> {
        let env = self.environment;

        if !Indicators::<Cache>::get_active_indicators_status_cache(&env).await {
            let mut strategies_request = Strategies::default();
            strategies_request.environment = env;

            let active_strategies = strategies_request
                .get_active_strategies()
                .await
                .unwrap_or_default();

            let mut indicators_request = Indicators::default();
            indicators_request.model.is_active = Some(true);

            let active_indicators = indicators_request
                .select_indicators()
                .await?
                .into_iter()
                .filter(|act_ind| active_strategies.contains_key(&act_ind.strategy_id))
                .collect::<Vec<_>>();

            Indicators::<Cache>::set_active_indicators_cache(&env, active_indicators).await;
        }

        Ok(())
    }

    pub async fn stop_active_indicators_core(self) {
        let env = self.environment;

        Indicators::<Cache>::stop_active_indicators_cache(&env).await
    }

    // misc
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
