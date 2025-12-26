use crate::{handler::DBC, utils::Cache};

use std::sync::Arc;

use models::structs::DatabaseManager;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

static ACTIVE_DATABASE: Lazy<Arc<RwLock<Option<DatabaseManager>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl DBC<Cache> {
    pub async fn set_database_cache(database_manager: DatabaseManager) -> DatabaseManager {
        let mut active_database_manager = ACTIVE_DATABASE.write().await;

        *active_database_manager = Some(database_manager.clone());

        database_manager
    }

    pub async fn get_database_cache() -> Option<DatabaseManager> {
        let active_database_manager = ACTIVE_DATABASE.read().await;

        active_database_manager.clone()
    }
}
