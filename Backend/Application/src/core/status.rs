use models::entities::status::Model;

use crate::{
    config::get_config,
    types::OrderStatus,
    utils::{Core, Response},
};

impl OrderStatus<Core> {
    pub async fn select_status_core(self) -> Result<Vec<Model>, Response> {
        self.next_phase()
            .select_status_data(&get_config().await.db)
            .await
    }
}
