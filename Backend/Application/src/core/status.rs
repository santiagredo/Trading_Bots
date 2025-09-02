use std::collections::HashMap;

use models::{entities::status::Model, enums::Status};

use crate::{
    config::get_config,
    handler::OrderStatus,
    utils::{Cache, Core, Response},
};

impl OrderStatus<Core> {
    pub async fn get_active_status_core() -> Option<HashMap<Status, i32>> {
        OrderStatus::<Cache>::get_active_status_cache().await
    }

    pub async fn start_active_status_core(self) -> Result<(), Response> {
        OrderStatus::<Cache>::start_active_status_cache().await
    }

    pub async fn stop_active_status_core(self) {
        OrderStatus::<Cache>::stop_active_status_cache().await
    }

    pub async fn select_status_core(self) -> Result<Vec<Model>, Response> {
        self.next_phase()
            .select_status_data(&get_config().await.db)
            .await
    }
}
