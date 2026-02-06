use application::{handler::Actions, utils::EntityCache};
use models::{entities::actions::Model, enums::LifecycleState, structs::Environments};
use sea_orm::prelude::Decimal;

fn mock_action(id: i32) -> Model {
    Model {
        id,
        strategy_id: id,
        is_active: true,
        is_sell: false,
        is_quote_asset: false,
        is_percentage: false,
        value: Decimal::new(50, 0),
        pair_id: 1,
        ..Default::default()
    }
}

#[tokio::test]
async fn full_actions_cache_flow_should_work_correctly() {
    let env = Environments::DEV;
    let service = Actions::blank();

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
     * START ACTIONS
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
     * LOAD MULTIPLE ACTIONS
     * ===========================
     */

    let actions = vec![mock_action(1), mock_action(2)];

    service.set_all(env, actions).await.unwrap();

    let cache = service.get_all(env).await.unwrap();
    assert_eq!(cache.models.len(), 2);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * INSERT INDIVIDUAL ACTION
     * ===========================
     */

    let extra = mock_action(3);
    service.upsert(env, 3, extra.clone()).await.unwrap();

    let single = service.get(env, 3).await.unwrap();
    assert_eq!(single, extra);

    /* ===========================
     * STOP
     * ===========================
     */

    service.stop(env).await.unwrap();

    let final_cache = service.get_all(env).await;
    assert!(final_cache.is_none());
}
