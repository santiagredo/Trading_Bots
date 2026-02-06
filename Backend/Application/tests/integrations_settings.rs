use application::{handler::IntegrationsSettings, utils::EntityCache};
use models::{entities::integration_settings::Model, enums::LifecycleState, structs::Environments};

fn mock_integration_setting(id: i32, integration_id: i32) -> Model {
    Model {
        id,
        integration_id,
        ..Default::default()
    }
}

#[tokio::test]
async fn full_integrations_settings_cache_flow_should_work_correctly() {
    let env = Environments::DEV;
    let service = IntegrationsSettings::blank();

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    service.reset(env).await.unwrap();

    assert_eq!(service.state(env).await, LifecycleState::Off);

    // Off => cache inaccesible
    assert!(service.get_all(env).await.is_none());

    /* ===========================
     * START CACHE
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

    /* ===========================
     * LOAD MULTIPLE SETTINGS
     * ===========================
     */

    let settings = vec![
        mock_integration_setting(1, 10),
        mock_integration_setting(2, 10),
        mock_integration_setting(3, 20),
    ];

    service.set_all(env, settings).await.unwrap();

    let cache = service.get_all(env).await.unwrap();

    assert_eq!(cache.integrations_map.len(), 2);
    assert_eq!(cache.integrations_map.get(&10).unwrap().models.len(), 2);
    assert_eq!(cache.integrations_map.get(&20).unwrap().models.len(), 1);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * GET SINGLE SETTINGS
     * ===========================
     */

    let s1 = service.get(env, 1).await.unwrap();
    let s2 = service.get(env, 2).await.unwrap();
    let s3 = service.get(env, 3).await.unwrap();

    assert_eq!(s1.integration_id, 10);
    assert_eq!(s2.integration_id, 10);
    assert_eq!(s3.integration_id, 20);

    /* ===========================
     * UPSERT
     * ===========================
     */

    let extra = mock_integration_setting(4, 20);

    service.upsert(env, 4, extra.clone()).await.unwrap();

    let single = service.get(env, 4).await.unwrap();

    assert_eq!(single, extra);

    let cache = service.get_all(env).await.unwrap();
    assert_eq!(cache.integrations_map.get(&20).unwrap().models.len(), 2);

    /* ===========================
     * STOP
     * ===========================
     */

    service
        .set_state(env, LifecycleState::Stopping)
        .await
        .unwrap();

    let removed = service.remove_all(env).await.unwrap();

    assert_eq!(removed.len(), 4);

    service.set_state(env, LifecycleState::Off).await.unwrap();

    // Off => cache inaccesible
    assert!(service.get_all(env).await.is_none());
}
