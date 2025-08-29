use chrono::NaiveDateTime;
use models::entities::strategies::Model;

use crate::{
    config::get_config,
    types::Strategies,
    utils::{handle_user_err, Data, Logic, Response},
};

impl<Core> Strategies<Core> {
    pub async fn insert_strategy_core(self) -> Result<Model, Response> {
        let logic_type = self
            .next_phase::<Logic>()
            .insert_strategy_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .insert_strategy_data(&get_config().await.db)
            .await
    }

    pub async fn select_strategy_core(self) -> Result<Option<Model>, Response> {
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
        let logic_type = self
            .next_phase::<Logic>()
            .update_strategy_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .update_strategy_data(&get_config().await.db)
            .await
    }

    pub async fn delete_strategy_core(self) -> Result<u64, Response> {
        let logic_type = self
            .next_phase::<Logic>()
            .delete_strategy_logic()
            .map_err(handle_user_err)?;

        logic_type
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
