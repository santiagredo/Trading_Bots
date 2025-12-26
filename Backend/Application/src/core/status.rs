use std::collections::HashMap;

use models::{entities::status::Model, enums::Status};

use crate::{
    handler::{OrderStatus, DBC},
    utils::{Cache, Core, Response},
};

impl OrderStatus<Core> {
    pub async fn get_active_status_core(self) -> Option<HashMap<Status, i32>> {
        let env = self.environment;
        OrderStatus::<Cache>::get_active_status_cache(&env).await
    }

    pub async fn start_active_status_core(self) -> Result<(), Response> {
        let env = self.environment;

        if !OrderStatus::<Cache>::get_active_status_status_cache(&env).await {
            let status_request = OrderStatus::default().with_env(&env);
            let status = status_request.select_status().await?;

            OrderStatus::<Cache>::set_active_status_cache(&env, status).await;
        }

        Ok(())
    }

    pub async fn stop_active_status_core(self) {
        let env = self.environment;

        OrderStatus::<Cache>::stop_active_status_cache(&env).await
    }

    pub async fn select_status_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase()
            .select_status_data(&DBC::db(&env).await?)
            .await
    }
}
