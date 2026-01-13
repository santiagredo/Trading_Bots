use application::{handler::Assets, utils::Cache};
use models::{entities::assets::Model, enums::LifecycleState, structs::Environments};
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

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    let reset = Assets::<Cache>::reset_assets_cache(env).await;
    assert!(reset.is_ok());

    let off = Assets::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    assert!(off.is_err());

    let status = Assets::default().with_env(env).get_assets_state().await;
    assert_eq!(status, LifecycleState::Off);

    let cache = Assets::<Cache>::get_assets_cache(env).await;
    assert!(cache.is_some_and(|val| val.models.is_empty()));

    /* ===========================
     * LOAD MULTIPLE ASSETS
     * ===========================
     */

    let starting = Assets::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
    assert!(starting.is_ok());

    let assets = vec![mock_asset(1, 100, 0), mock_asset(2, 200, 20)];

    let running = Assets::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    assert!(running.is_ok());

    Assets::<Cache>::set_assets_cache(env, assets)
        .await
        .unwrap();

    let cache = Assets::<Cache>::get_assets_cache(env).await.unwrap();
    assert_eq!(cache.models.len(), 2);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * INSERT INDIVIDUAL ASSET
     * ===========================
     */

    let extra = mock_asset(3, 50, 0);
    Assets::<Cache>::upsert_asset_cache(env, extra.clone())
        .await
        .unwrap();

    let single = Assets::<Cache>::get_asset_cache(env, 3).await.unwrap();
    assert_eq!(single, extra);

    /* ===========================
     * UPDATE BALANCE
     * ===========================
     */

    let (updated, previous) =
        Assets::<Cache>::update_asset_balance_cache(env, 3, Decimal::new(25, 0), false, false)
            .await
            .unwrap();

    assert_eq!(previous, Decimal::new(50, 0));
    assert_eq!(updated.free, Decimal::new(75, 0));

    /* ===========================
     * STOP
     * ===========================
     */

    Assets::<Cache>::set_status_cache(env, LifecycleState::Stopping)
        .await
        .unwrap();

    Assets::<Cache>::remove_assets_cache(env).await.unwrap();

    Assets::<Cache>::set_status_cache(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = Assets::<Cache>::get_assets_cache(env).await;
    assert!(final_cache.is_some_and(|val| val.models.is_empty()));
}
