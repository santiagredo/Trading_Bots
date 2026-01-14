use application::{handler::Integrations, utils::Cache};
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

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    let reset = Integrations::<Cache>::reset_integrations_cache(env).await;
    assert!(reset.is_ok());

    let off = Integrations::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    assert!(off.is_err());

    let status = Integrations::<Cache>::get_cache_state(env).await;
    assert_eq!(status, LifecycleState::Off);

    let cache = Integrations::<Cache>::get_integrations_cache(env).await;
    assert!(cache.is_some_and(|val| val.models.is_empty()));

    /* ===========================
     * LOAD MULTIPLE INTEGRATIONS
     * ===========================
     */

    let starting = Integrations::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
    assert!(starting.is_ok());

    let integrations = vec![mock_integration(1), mock_integration(2)];

    let running = Integrations::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    assert!(running.is_ok());

    Integrations::<Cache>::set_integrations_cache(env, integrations)
        .await
        .unwrap();

    let cache = Integrations::<Cache>::get_integrations_cache(env)
        .await
        .unwrap();
    assert_eq!(cache.models.len(), 2);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * INSERT INDIVIDUAL INTEGRATION
     * ===========================
     */

    let extra = mock_integration(3);
    Integrations::<Cache>::upsert_integration_cache(env, extra.clone())
        .await
        .unwrap();

    let single = Integrations::<Cache>::get_integration_cache(env, 3)
        .await
        .unwrap();
    assert_eq!(single, extra);

    /* ===========================
     * STOP
     * ===========================
     */

    Integrations::<Cache>::set_status_cache(env, LifecycleState::Stopping)
        .await
        .unwrap();

    Integrations::<Cache>::remove_integrations_cache(env)
        .await
        .unwrap();

    Integrations::<Cache>::set_status_cache(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = Integrations::<Cache>::get_integrations_cache(env).await;
    assert!(final_cache.is_some_and(|val| val.models.is_empty()));
}
