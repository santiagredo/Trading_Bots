use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use models::{entities::status::Model, enums::Status};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::utils::{Response, Types};

#[derive(Debug, Default)]
pub struct OrderStatus<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: Status,
}

static ACTIVE_STATUS: Lazy<Arc<RwLock<Option<HashMap<Status, i32>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl OrderStatus {
    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: Status::default(),
        }
    }

    pub async fn set_active_status(status: Option<Vec<Model>>) -> Option<Vec<Model>> {
        let mut active_status = ACTIVE_STATUS.write().await;

        let Some(status) = status else {
            *active_status = None;
            return None;
        };

        let mut active_status_map: HashMap<Status, i32> = HashMap::new();

        for sta in status.iter() {
            active_status_map.insert(Status::from_model(&sta), sta.id);
        }

        *active_status = Some(active_status_map);

        Some(status)
    }

    pub async fn get_active_status() -> Option<HashMap<Status, i32>> {
        let active_status = ACTIVE_STATUS.read().await;

        active_status.clone()
    }

    pub async fn start_active_status() -> Result<(), Response> {
        if Self::get_active_status()
            .await
            .is_none_or(|status| status.is_empty())
        {
            let status = Self::default().select_status().await?;
            Self::set_active_status(Some(status)).await;
        }

        Ok(())
    }

    pub async fn stop_active_status() {
        Self::set_active_status(None).await;
    }
}

impl<Phase> OrderStatus<Phase> {
    pub fn next_phase<Next>(self) -> OrderStatus<Next> {
        OrderStatus {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl OrderStatus<Types> {
    pub async fn select_status(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_status_core().await
    }
}
