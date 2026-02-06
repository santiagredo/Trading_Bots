use application::{handler::Indicators, utils::EntityCache};
use models::{entities::indicators::Model, enums::LifecycleState, structs::Environments};

fn mock_indicator(id: i32) -> Model {
    Model {
        id,
        strategy_id: id,
        is_active: true,
        ..Default::default()
    }
}

#[tokio::test]
async fn full_indicators_cache_flow_should_work_correctly() {
    let env = Environments::DEV;
    let service = Indicators::blank();

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    service.reset_indicators(env).await.unwrap();

    let status = service.state(env).await;
    assert_eq!(status, LifecycleState::Off);

    let cache = service.get_all(env).await;
    assert!(cache.is_none());

    /* ===========================
     * START INDICATORS
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
    assert!(cache.models.is_empty());

    /* ===========================
     * LOAD MULTIPLE INDICATORS
     * ===========================
     */

    let indicators = vec![mock_indicator(1), mock_indicator(2)];

    service.set_all(env, indicators).await.unwrap();

    let cache = service.get_all(env).await.unwrap();
    assert_eq!(cache.models.len(), 2);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * INSERT INDIVIDUAL INDICATOR
     * ===========================
     */

    let extra = mock_indicator(3);

    service.upsert(env, extra.id, extra.clone()).await.unwrap();

    let single = service.get(env, 3).await.unwrap();
    assert_eq!(single, extra);

    /* ===========================
     * STOP
     * ===========================
     */

    service
        .set_state(env, LifecycleState::Stopping)
        .await
        .unwrap();

    let removed = service.remove_all(env).await.unwrap();
    assert_eq!(removed.len(), 3);

    service.set_state(env, LifecycleState::Off).await.unwrap();

    let final_cache = service.get_all(env).await;
    assert!(final_cache.is_none());
}
