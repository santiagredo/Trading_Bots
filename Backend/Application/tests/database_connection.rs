use sea_orm::{Database, DbConn};

use application::{handler::DBC, utils::Cache};
use models::structs::DatabaseManager;

async fn db() -> DbConn {
    Database::connect("sqlite::memory:").await.unwrap()
}

#[tokio::test]
async fn full_database_cache_flow_should_work_correctly() {
    // Initial state
    let initial = DBC::<Cache>::get_database_cache().await;
    assert!(initial.is_none());

    // Insert database manager
    let manager = DatabaseManager {
        dev: db().await,
        prod: db().await,
    };

    DBC::<Cache>::set_database_cache(manager.clone()).await;

    let cached = DBC::<Cache>::get_database_cache()
        .await
        .expect("database manager should exist");

    assert!(cached.dev.ping().await.is_ok());
    assert!(cached.prod.ping().await.is_ok());

    // Override database manager
    let new_manager = DatabaseManager {
        dev: db().await,
        prod: db().await,
    };

    DBC::<Cache>::set_database_cache(new_manager).await;

    let cached_after = DBC::<Cache>::get_database_cache()
        .await
        .expect("database manager should exist");

    assert!(cached_after.dev.ping().await.is_ok());
    assert!(cached_after.prod.ping().await.is_ok());
}
