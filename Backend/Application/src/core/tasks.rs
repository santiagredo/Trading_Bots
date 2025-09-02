use crate::{
    handler::Tasks,
    utils::{Cache, Core},
};

impl Tasks<Core> {
    pub async fn start_async_tasks_core() {
        Tasks::<Cache>::start_async_tasks_cache().await
    }

    pub async fn stop_async_tasks_core() {
        Tasks::<Cache>::stop_async_tasks_cache().await
    }
}
