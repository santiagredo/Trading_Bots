use std::{collections::HashMap, sync::Arc};

use models::{entities::status::Model, enums::Status};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{
    handler::OrderStatus,
    utils::{Cache, Response},
};

static ACTIVE_STATUS: Lazy<Arc<RwLock<Option<HashMap<Status, i32>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl OrderStatus<Cache> {
    pub async fn set_active_status_cache(status: Option<Vec<Model>>) -> Option<Vec<Model>> {
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

    pub async fn get_active_status_cache() -> Option<HashMap<Status, i32>> {
        let active_status = ACTIVE_STATUS.read().await;

        active_status.clone()
    }

    pub async fn start_active_status_cache() -> Result<(), Response> {
        if OrderStatus::<Cache>::get_active_status_cache()
            .await
            .is_none_or(|status| status.is_empty())
        {
            let status = OrderStatus::default().select_status().await?;
            OrderStatus::<Cache>::set_active_status_cache(Some(status)).await;
        }

        Ok(())
    }

    pub async fn stop_active_status_cache() {
        OrderStatus::<Cache>::set_active_status_cache(None).await;
    }
}
