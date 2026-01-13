use std::{collections::HashMap, marker::PhantomData};

use models::{
    entities::status::Model,
    enums::{LifecycleState, Status},
    structs::Environments,
};

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct OrderStatus<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: Status,
}

impl<Phase> OrderStatus<Phase> {
    pub fn next_phase<Next>(self) -> OrderStatus<Next> {
        OrderStatus {
            phase: PhantomData::<Next>,
            environment: self.environment,
            model: self.model,
        }
    }
}

impl OrderStatus {
    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model: Status::default(),
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment: environment,
            model: self.model,
        }
    }

    pub async fn get_statuses(self) -> Option<HashMap<i32, Model>> {
        self.next_phase().get_statuses_core().await
    }

    pub async fn start_status(self) -> Result<(), Response> {
        self.next_phase().start_status_core().await
    }

    pub async fn stop_status(self) -> Result<(), Response> {
        self.next_phase().stop_status_core().await
    }

    pub async fn select_status(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_status_core().await
    }

    pub async fn reset_status(self) -> Result<(), Response> {
        self.next_phase().reset_status_core().await
    }

    pub async fn get_status_state(self) -> LifecycleState {
        self.next_phase().get_status_state_core().await
    }
}
