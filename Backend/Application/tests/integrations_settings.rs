use application::{handler::IntegrationsSettings, utils::Cache};
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

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    let reset = IntegrationsSettings::<Cache>::reset_integrations_settings_cache(env).await;
    assert!(reset.is_ok());

    let off = IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    assert!(off.is_err());

    let status = IntegrationsSettings::<Cache>::get_cache_state(env).await;
    assert_eq!(status, LifecycleState::Off);

    let cache = IntegrationsSettings::<Cache>::get_integrations_settings_cache(env).await;
    assert!(cache.is_some_and(|val| val.integrations_map.is_empty()));

    /* ===========================
     * LOAD MULTIPLE INTEGRATION SETTINGS
     * ===========================
     */

    IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Starting)
        .await
        .unwrap();

    IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Running)
        .await
        .unwrap();

    let settings = vec![
        mock_integration_setting(1, 10),
        mock_integration_setting(2, 10),
        mock_integration_setting(3, 20),
    ];

    IntegrationsSettings::<Cache>::set_integrations_settings_cache(env, settings)
        .await
        .unwrap();

    let cache = IntegrationsSettings::<Cache>::get_integrations_settings_cache(env)
        .await
        .unwrap();

    assert_eq!(cache.integrations_map.len(), 2);
    assert_eq!(cache.integrations_map.get(&10).unwrap().models.len(), 2);
    assert_eq!(cache.integrations_map.get(&20).unwrap().models.len(), 1);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * GET SETTINGS BY INTEGRATION ID
     * ===========================
     */

    let integration_10 = IntegrationsSettings::<Cache>::get_integration_settings_cache(env, 10)
        .await
        .unwrap();

    assert_eq!(integration_10.len(), 2);
    assert!(integration_10.contains_key(&1));
    assert!(integration_10.contains_key(&2));

    let integration_20 = IntegrationsSettings::<Cache>::get_integration_settings_cache(env, 20)
        .await
        .unwrap();

    assert_eq!(integration_20.len(), 1);
    assert!(integration_20.contains_key(&3));

    /* ===========================
     * INSERT INDIVIDUAL INTEGRATION SETTING
     * ===========================
     */

    let extra = mock_integration_setting(4, 20);

    IntegrationsSettings::<Cache>::upsert_integration_setting_cache(env, extra.clone())
        .await
        .unwrap();

    let single = IntegrationsSettings::<Cache>::get_integration_setting_cache(env, 4)
        .await
        .unwrap();

    assert_eq!(single, extra);

    // validate integration_id = 20 updated correctly
    let integration_20_after =
        IntegrationsSettings::<Cache>::get_integration_settings_cache(env, 20)
            .await
            .unwrap();

    assert_eq!(integration_20_after.len(), 2);
    assert!(integration_20_after.contains_key(&3));
    assert!(integration_20_after.contains_key(&4));

    /* ===========================
     * STOP
     * ===========================
     */

    IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Stopping)
        .await
        .unwrap();

    let removed = IntegrationsSettings::<Cache>::remove_integrations_settings_cache(env)
        .await
        .unwrap();

    assert!(!removed.is_empty());

    IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = IntegrationsSettings::<Cache>::get_integrations_settings_cache(env).await;

    assert!(final_cache.is_some_and(|val| val.integrations_map.is_empty()));
}
