use std::marker::PhantomData;

use crate::utils::{Core, Types};

#[derive(Debug, Default)]
pub struct Tasks<Phase = Types> {
    phase: PhantomData<Phase>,
}

impl Tasks {
    pub async fn start_async_tasks() {
        Tasks::<Core>::start_async_tasks_core().await
    }

    pub async fn stop_async_tasks() {
        Tasks::<Core>::stop_async_tasks_core().await
    }
}
