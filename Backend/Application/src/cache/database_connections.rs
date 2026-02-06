use crate::handler::DBC;

use std::sync::Arc;

use models::structs::DatabaseManager;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

static ACTIVE_DATABASE: Lazy<Arc<RwLock<Option<DatabaseManager>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl DBC {
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

#[cfg(test)]
mod tests {
    use sea_orm::{Database, DbConn};

    use crate::handler::DBC;
    use models::structs::DatabaseManager;

    // Helpers
    async fn mock_db_conn() -> DbConn {
        Database::connect("sqlite::memory:").await.unwrap()
    }

    async fn mock_database_manager() -> DatabaseManager {
        DatabaseManager {
            dev: mock_db_conn().await,
            prod: mock_db_conn().await,
        }
    }

    async fn reset_database_cache() {
        // Overwrite cache with None
        let mut lock = super::ACTIVE_DATABASE.write().await;
        *lock = None;
    }

    // Scenarios (unit responsibilities)

    // Initial insertion
    async fn scenario_set_database_cache() {
        reset_database_cache().await;

        let db_manager = mock_database_manager().await;
        let returned = DBC::set_database_cache(db_manager.clone()).await;

        assert!(returned.dev.ping().await.is_ok());
        assert!(returned.prod.ping().await.is_ok())
    }

    // Get cache
    async fn scenario_get_database_cache() {
        reset_database_cache().await;

        let db_manager = mock_database_manager().await;
        DBC::set_database_cache(db_manager.clone()).await;

        let cached = DBC::get_database_cache()
            .await
            .expect("database manager should exist");

        // DbConn can't be compared
        // Check if they exist and are connected
        assert!(cached.dev.ping().await.is_ok());
        assert!(cached.prod.ping().await.is_ok());
    }

    // Get without initialize
    async fn scenario_get_database_cache_empty() {
        reset_database_cache().await;

        let cached = DBC::get_database_cache().await;
        assert!(cached.is_none());
    }

    // Replace existing cache
    async fn scenario_override_database_cache() {
        reset_database_cache().await;

        let first = mock_database_manager().await;
        DBC::set_database_cache(first).await;

        let second = mock_database_manager().await;
        DBC::set_database_cache(second).await;

        let cached = DBC::get_database_cache().await.unwrap();

        assert!(cached.dev.ping().await.is_ok());
        assert!(cached.prod.ping().await.is_ok());
    }

    #[tokio::test]
    async fn database_cache_unit_responsibilities() {
        scenario_get_database_cache_empty().await;
        scenario_set_database_cache().await;
        scenario_get_database_cache().await;
        scenario_override_database_cache().await;

        reset_database_cache().await;
    }
}
