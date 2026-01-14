use std::marker::PhantomData;

use models::{
    entities::tasks::Model,
    enums::LifecycleState,
    structs::{CacheTask, CacheTasks, Environments, QueryOptions, TaskRequest},
};
use tokio_util::sync::CancellationToken;

use crate::{
    handler::Cancellations,
    utils::{Response, Types},
};

#[derive(Debug, Default)]
pub struct Tasks<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: TaskRequest,
}

impl<Phase> Tasks<Phase> {
    pub fn next_phase<Next>(self) -> Tasks<Next> {
        Tasks {
            phase: PhantomData::<Next>,
            environment: self.environment,
            model: self.model,
        }
    }
}

impl Tasks {
    pub fn new(model: TaskRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model: TaskRequest {
                ..Default::default()
            },
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment,
            model: self.model,
        }
    }

    // db
    pub async fn select_tasks(self, query: Option<QueryOptions>) -> Result<Vec<Model>, Response> {
        self.next_phase().select_tasks_core(query).await
    }

    pub async fn update_task(self) -> Result<Model, Response> {
        self.next_phase().update_task_core().await
    }

    // cache
    pub async fn get_tasks(self) -> Option<CacheTasks> {
        self.next_phase().get_tasks_core().await
    }

    pub async fn get_task(self, task_id: i32) -> Option<CacheTask> {
        self.next_phase().get_task_core(task_id).await
    }

    pub async fn get_tasks_state(self) -> LifecycleState {
        self.next_phase().get_tasks_state_core().await
    }

    pub async fn upsert_task(self, task: Model) -> Result<(), Response> {
        self.next_phase().upsert_task_core(task).await
    }

    pub async fn remove_task(self, task_id: i32) -> Result<Option<CacheTask>, Response> {
        self.next_phase().remove_task_core(task_id).await
    }

    pub async fn reset_tasks(self) -> Result<(), Response> {
        self.next_phase().reset_tasks_core().await
    }

    pub async fn start_tasks(self, token: &CancellationToken) -> Result<(), Response> {
        self.next_phase().start_tasks_core(token).await
    }

    pub async fn start_tasks_manually(self) -> Result<(), Response> {
        let runtime_token = match Cancellations::new(self.environment)
            .get_runtime_token()
            .await
        {
            Some(val) => val,
            None => Cancellations::new(self.environment).start_runtime().await?,
        };

        self.start_tasks(&runtime_token).await
    }

    pub async fn stop_tasks(self) -> Result<(), Response> {
        self.next_phase().stop_tasks_core().await
    }
}
