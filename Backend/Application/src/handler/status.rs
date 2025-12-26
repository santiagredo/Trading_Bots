use std::{collections::HashMap, marker::PhantomData};

use models::{entities::status::Model, enums::Status, structs::Environments};

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

    pub fn with_env(self, environment: &Environments) -> Self {
        Self {
            phase: self.phase,
            environment: *environment,
            model: self.model,
        }
    }

    pub async fn get_active_status(self) -> Option<HashMap<Status, i32>> {
        self.next_phase().get_active_status_core().await
    }

    pub async fn start_active_status(self) -> Result<(), Response> {
        self.next_phase().start_active_status_core().await
    }

    pub async fn stop_active_status(self) {
        self.next_phase().stop_active_status_core().await
    }

    pub async fn select_status(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_status_core().await
    }
}
