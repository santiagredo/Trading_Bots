use std::collections::HashMap;

use chrono::NaiveDateTime;
use models::entities::strategies::Model;

use crate::{
    config::get_config,
    handler::Strategies,
    utils::{handle_user_err, Cache, Data, Logic, Response},
};

impl<Core> Strategies<Core> {
    pub async fn get_active_strategies_core() -> Option<HashMap<i32, Model>> {
        Strategies::<Cache>::get_active_strategies_cache().await
    }

    pub async fn start_active_strategies_core() -> Result<(), Response> {
        Strategies::<Cache>::start_active_strategies_cache().await
    }

    pub async fn stop_active_strategies_core() {
        Strategies::<Cache>::stop_active_strategies_cache().await
    }

    pub async fn start_strategies_evaluation_loop_core() {
        Strategies::<Cache>::start_strategies_evaluation_loop_cache().await
    }

    pub async fn stop_strategies_evaluation_loop_core() {
        Strategies::<Cache>::stop_strategies_evaluation_loop_cache().await
    }

    pub async fn insert_strategy_core(self) -> Result<Model, Response> {
        let strategy = self
            .next_phase::<Logic>()
            .insert_strategy_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .insert_strategy_data(&get_config().await.db)
            .await?;

        Ok(Strategies::<Cache>::set_active_strategy_cache(strategy).await)
    }

    pub async fn select_strategy_core(self) -> Result<Option<Model>, Response> {
        let memory_strategy =
            Strategies::<Cache>::get_active_strategy_cache(&self.model.id.unwrap_or_default())
                .await;

        if memory_strategy.is_some() {
            return Ok(memory_strategy);
        }

        self.next_phase::<Data>()
            .select_strategy_data(&get_config().await.db)
            .await
    }

    pub async fn select_strategies_core(self) -> Result<Vec<Model>, Response> {
        self.next_phase::<Data>()
            .select_strategies_data(&get_config().await.db)
            .await
    }

    pub async fn update_strategy_core(self) -> Result<Model, Response> {
        let strategy = self
            .next_phase::<Logic>()
            .update_strategy_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .update_strategy_data(&get_config().await.db)
            .await?;

        Ok(Strategies::<Cache>::set_active_strategy_cache(strategy).await)
    }

    pub async fn delete_strategy_core(self) -> Result<u64, Response> {
        let mut strategy = Strategies::into_model(self.model.clone());
        strategy.is_active = false;

        Strategies::<Cache>::set_active_strategy_cache(strategy).await;

        self.next_phase::<Logic>()
            .delete_strategy_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .delete_strategy_data(&get_config().await.db)
            .await
    }

    pub fn evaluate_cooldown_core(
        self,
        last_exec: Option<NaiveDateTime>,
        cooldown: Option<i32>,
    ) -> bool {
        self.next_phase()
            .evaluate_cooldown_logic(last_exec, cooldown)
    }
}
