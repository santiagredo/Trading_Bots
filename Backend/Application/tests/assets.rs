use application::{handler::Assets, utils::EntityCache};
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
    let service = Assets::blank();

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    service.reset(env).await.unwrap();

    let status = service.state(env).await;
    assert_eq!(status, LifecycleState::Off);

    let cache = service.get_all(env).await;
    assert!(cache.is_none());

    /* ===========================
     * START ASSETS
     * ===========================
     */

    service
        .set_state(env, LifecycleState::Starting)
        .await
        .unwrap();

    service
        .set_state(env, LifecycleState::Running)
        .await
        .unwrap();

    let cache = service.get_all(env).await.unwrap();
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * INSERT INDIVIDUAL ASSET
     * ===========================
     */

    let extra = mock_asset(1, 50, 0);
    service.upsert(env, 1, extra.clone()).await.unwrap();

    let single = service.get(env, 1).await.unwrap();
    assert_eq!(single, extra);

    /* ===========================
     * UPDATE BALANCE
     * ===========================
     */

    let (updated, previous) = service
        .update_asset_balance_cache(env, 1, Decimal::new(25, 0), false, false)
        .await
        .unwrap();

    assert_eq!(previous, Decimal::new(50, 0));
    assert_eq!(updated.free, Decimal::new(75, 0));

    /* ===========================
     * REMOVE INDIVIDUAL ASSET
     * ===========================
     */

    let removed = service.remove(env, 1).await.unwrap();
    assert!(removed.is_some());

    let check_removed = service.get(env, 1).await;
    assert!(check_removed.is_none());

    /* ===========================
     * STOP
     * ===========================
     */

    service.stop(env).await.unwrap();

    let final_cache = service.get_all(env).await;
    assert!(final_cache.is_none());
}
