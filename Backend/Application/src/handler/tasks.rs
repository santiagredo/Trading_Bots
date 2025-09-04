use std::marker::PhantomData;

use models::{entities::tasks::Model, structs::TaskRequest};

use crate::utils::{Core, Response, Types};

#[derive(Debug, Default)]
pub struct Tasks<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub model: TaskRequest,
}

impl<Phase> Tasks<Phase> {
    pub fn next_phase<Next>(self) -> Tasks<Next> {
        Tasks {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Tasks {
    pub fn new(model: TaskRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: TaskRequest {
                ..Default::default()
            },
        }
    }

    pub async fn start_async_tasks() -> Result<(), Response> {
        Tasks::<Core>::start_async_tasks_core().await
    }

    pub async fn stop_async_tasks() {
        Tasks::<Core>::stop_async_tasks_core().await
    }

    pub async fn select_tasks(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_tasks_core().await
    }

    pub async fn select_active_tasks(self) -> Option<Vec<Model>> {
        self.next_phase().select_active_tasks_core().await
    }
}
