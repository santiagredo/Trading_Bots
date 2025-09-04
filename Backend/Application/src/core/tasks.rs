use models::entities::tasks::Model;

use crate::{
    config::get_config,
    handler::Tasks,
    utils::{Cache, Core, Response},
};

impl Tasks<Core> {
    pub async fn start_async_tasks_core() -> Result<(), Response> {
        Tasks::<Cache>::start_async_tasks_cache().await
    }

    pub async fn stop_async_tasks_core() {
        Tasks::<Cache>::stop_async_tasks_cache().await
    }

    pub async fn select_tasks_core(self) -> Result<Vec<Model>, Response> {
        self.next_phase()
            .select_tasks_data(&get_config().await.db)
            .await
    }

    pub async fn select_active_tasks_core(self) -> Option<Vec<Model>> {
        Tasks::get_active_tasks_cache().await
    }
}
