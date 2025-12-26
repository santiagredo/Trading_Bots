use models::entities::orders::Model;

use crate::{
    handler::{Orders, DBC},
    utils::{handle_user_err, Core, Data, Logic, Response},
};

impl Orders<Core> {
    pub async fn insert_order_core(self) -> Result<Model, Response> {
        let env = self.environment;

        let logic_type = self
            .next_phase::<Logic>()
            .insert_order_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase()
            .insert_order_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_order_core(self) -> Result<Model, Response> {
        let env = self.environment;

        let logic_type = self
            .next_phase::<Logic>()
            .select_order_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .select_order_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn update_order_core(self) -> Result<Model, Response> {
        let env = self.environment;

        let logic_type = self
            .next_phase::<Logic>()
            .update_order_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .update_order_data(&DBC::db(&env).await?)
            .await
    }
}
