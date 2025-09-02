use std::{collections::HashMap, marker::PhantomData};

use models::{entities::status::Model, enums::Status};

use crate::utils::{Core, Response, Types};

#[derive(Debug, Default)]
pub struct OrderStatus<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: Status,
}

impl<Phase> OrderStatus<Phase> {
    pub fn next_phase<Next>(self) -> OrderStatus<Next> {
        OrderStatus {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl OrderStatus {
    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: Status::default(),
        }
    }

    pub async fn get_active_status() -> Option<HashMap<Status, i32>> {
        OrderStatus::<Core>::get_active_status_core().await
    }

    pub async fn start_active_status() -> Result<(), Response> {
        Self::default()
            .next_phase()
            .start_active_status_core()
            .await
    }

    pub async fn stop_active_status() {
        Self::default().next_phase().stop_active_status_core().await
    }

    pub async fn select_status(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_status_core().await
    }
}
