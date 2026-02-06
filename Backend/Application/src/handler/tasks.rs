use crate::{
    handler::Cancellations,
    utils::{RepoFactory, Response},
};
use models::{
    entities::tasks::Model,
    enums::LifecycleState,
    structs::{CacheTask, CacheTasks, Environments, QueryOptions, TaskRequest},
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Default)]
pub struct Tasks;

impl Tasks {
    pub fn new() -> Self {
        Self
    }

    // db
    pub async fn select_tasks(
        self,
        env: Environments,
        task: TaskRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.select_tasks_core(env, task, query).await
    }

    pub async fn update_task(
        self,
        env: Environments,
        task: TaskRequest,
    ) -> Result<Model, Response> {
        self.update_task_core(env, task).await
    }

    // cache
    pub async fn get_tasks(self, env: Environments) -> Option<CacheTasks> {
        self.get_tasks_core(env).await
    }

    pub async fn get_task(self, env: Environments, task_id: i32) -> Option<CacheTask> {
        self.get_task_core(env, task_id).await
    }

    pub async fn get_tasks_state(self, env: Environments) -> LifecycleState {
        self.get_tasks_state_core(env).await
    }

    pub async fn upsert_task(self, env: Environments, task: Model) -> Result<(), Response> {
        self.upsert_task_core(env, task).await
    }

    pub async fn remove_task(
        self,
        env: Environments,
        task_id: i32,
    ) -> Result<Option<CacheTask>, Response> {
        self.remove_task_core(env, task_id).await
    }

    pub async fn reset_tasks(self, env: Environments) -> Result<(), Response> {
        self.reset_tasks_core(env).await
    }

    pub async fn start_tasks(
        self,
        factory: RepoFactory,
        env: Environments,
        token: &CancellationToken,
    ) -> Result<(), Response> {
        self.start_tasks_core(factory, env, token).await
    }

    pub async fn start_tasks_manually(
        self,
        factory: RepoFactory,
        env: Environments,
    ) -> Result<(), Response> {
        let runtime_token = match Cancellations::new().get_runtime_token(env).await {
            Some(val) => val,
            None => Cancellations::new().start_runtime(env).await?,
        };

        self.start_tasks(factory, env, &runtime_token).await
    }

    pub async fn stop_tasks(self, env: Environments) -> Result<(), Response> {
        self.stop_tasks_core(env).await
    }
}
