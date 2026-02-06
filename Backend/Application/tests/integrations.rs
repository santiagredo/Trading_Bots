use application::{handler::Integrations, utils::EntityCache};
use models::{entities::integrations::Model, enums::LifecycleState, structs::Environments};

fn mock_integration(id: i32) -> Model {
    Model {
        id,
        ..Default::default()
    }
}

#[tokio::test]
async fn full_integrations_cache_flow_should_work_correctly() {
    let env = Environments::DEV;

    let service = Integrations::blank();

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    service.reset(env).await.unwrap();

    let off = service.set_state(env, LifecycleState::Off).await;
    assert!(off.is_err());

    let status = service.state(env).await;
    assert_eq!(status, LifecycleState::Off);

    let cache = service.get_all(env).await;
    assert!(cache.is_none());

    /* ===========================
     * LOAD MULTIPLE INTEGRATIONS
     * ===========================
     */

    service
        .set_state(env, LifecycleState::Starting)
        .await
        .unwrap();

    let integrations = vec![mock_integration(1), mock_integration(2)];

    service
        .set_state(env, LifecycleState::Running)
        .await
        .unwrap();

    service.set_all(env, integrations).await.unwrap();

    let cache = service.get_all(env).await.unwrap();
    assert_eq!(cache.models.len(), 2);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * INSERT INDIVIDUAL INTEGRATION
     * ===========================
     */

    let extra = mock_integration(3);

    service.upsert(env, extra.id, extra.clone()).await.unwrap();

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
