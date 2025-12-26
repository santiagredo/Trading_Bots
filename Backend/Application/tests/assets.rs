use application::{handler::Assets, utils::Cache};
use models::{entities::assets::Model, structs::Environments};
use sea_orm::prelude::Decimal;

fn mock_asset(id: i32, free: i64, locked: i64) -> Model {
    Model {
        id,
        free: Decimal::new(free, 0),
        locked: Decimal::new(locked, 0),
        ..Default::default()
    }
}

#[tokio::test]
async fn full_assets_cache_flow_should_work_correctly() {
    let env = Environments::DEV;

    // Initial state
    let _ = Assets::<Cache>::stop_active_assets_cache(&env).await;
    assert!(!Assets::<Cache>::get_active_assets_status_cache(&env).await);

    // Load multiple assets
    let assets = vec![mock_asset(1, 100, 0), mock_asset(2, 200, 20)];

    Assets::<Cache>::set_active_assets_cache(&env, assets).await;

    let cache = Assets::<Cache>::get_active_assets_cache(&env)
        .await
        .unwrap();
    assert_eq!(cache.len(), 2);

    // Insert individual asset
    let extra = mock_asset(3, 50, 0);
    Assets::<Cache>::set_active_asset_cache(&env, extra.clone(), false).await;

    let single = Assets::<Cache>::get_active_asset_cache(&env, &3).await;
    assert_eq!(single.unwrap().model, extra);

    // Update balance
    let (updated, previous) =
        Assets::<Cache>::set_active_asset_value_cache(&env, &3, Decimal::new(25, 0), false, false)
            .await
            .unwrap();

    assert_eq!(previous, Decimal::new(50, 0));
    assert_eq!(updated.free, Decimal::new(75, 0));

    // Status is true
    assert!(Assets::<Cache>::get_active_assets_status_cache(&env).await);

    // Stop
    Assets::<Cache>::stop_active_assets_cache(&env)
        .await
        .unwrap();

    let final_cache = Assets::<Cache>::get_active_assets_cache(&env)
        .await
        .unwrap();

    assert!(final_cache.is_empty());
    assert!(!Assets::<Cache>::get_active_assets_status_cache(&env).await);
}
