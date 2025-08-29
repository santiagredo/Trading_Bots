use models::entities::orders::Model;

use crate::{
    config::get_config,
    types::Orders,
    utils::{handle_user_err, Core, Data, Logic, Response},
};

impl Orders<Core> {
    pub async fn insert_order_core(self) -> Result<Model, Response> {
        let logic_type = self
            .next_phase::<Logic>()
            .insert_order_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase()
            .insert_order_data(&get_config().await.db)
            .await
    }

    pub async fn select_order_core(self) -> Result<Model, Response> {
        let logic_type = self
            .next_phase::<Logic>()
            .select_order_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .select_order_data(&get_config().await.db)
            .await
    }

    // pub async fn select_open_orders() -> Outcome<Vec<Model>, String, String> {
    //     Orders::<Data>::select_open_orders(&get_config().await.db).await
    // }

    // pub async fn select_completed_orders() -> Outcome<Vec<Model>, String, String> {
    //     Orders::<Data>::select_completed_orders(&get_config().await.db).await
    // }

    pub async fn update_order_core(self) -> Result<Model, Response> {
        let logic_type = self
            .next_phase::<Logic>()
            .update_order_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .update_order_data(&get_config().await.db)
            .await
    }
}
