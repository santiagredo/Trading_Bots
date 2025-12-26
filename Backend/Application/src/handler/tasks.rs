use std::marker::PhantomData;

use models::{
    entities::tasks::Model,
    structs::{Environments, TaskRequest},
};

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct Tasks<Phase = Types> {
    pub phase: PhantomData<Phase>,
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

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment: environment,
            model: self.model,
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

    // db
    pub async fn select_tasks(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_tasks_core().await
    }

    pub async fn update_task(self) -> Result<Model, Response> {
        self.next_phase().update_task_core().await
    }

    // cache
    pub async fn get_active_tasks(self) -> Option<Vec<Model>> {
        self.next_phase().get_active_tasks_core().await
    }

    pub async fn start_active_tasks(self) -> Result<(), Response> {
        self.next_phase().start_active_tasks_core().await
    }

    pub async fn stop_active_tasks(self) {
        self.next_phase().stop_active_tasks_core().await
    }
}
