use application::{handler::Actions, utils::Cache};
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

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    let reset = Actions::<Cache>::reset_actions_cache(env).await;
    assert!(reset.is_ok());

    let off = Actions::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    assert!(off.is_err());

    let status = Actions::get_cache_state(env).await;
    assert_eq!(status, LifecycleState::Off);

    let cache = Actions::<Cache>::get_actions_cache(env).await;
    assert!(cache.is_some_and(|val| val.models.is_empty()));

    /* ===========================
     * LOAD MULTIPLE ACTIONS
     * ===========================
     */

    let starting = Actions::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
    assert!(starting.is_ok());

    let actions = vec![mock_action(1), mock_action(2)];

    let running = Actions::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    assert!(running.is_ok());

    Actions::<Cache>::set_actions_cache(env, actions)
        .await
        .unwrap();

    let cache = Actions::<Cache>::get_actions_cache(env).await.unwrap();
    assert_eq!(cache.models.len(), 2);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * INSERT INDIVIDUAL ACTION
     * ===========================
     */

    let extra = mock_action(3);
    Actions::<Cache>::upsert_action_cache(env, extra.clone())
        .await
        .unwrap();

    let single = Actions::<Cache>::get_action_cache(env, 3).await.unwrap();
    assert_eq!(single, extra);

    /* ===========================
     * STOP
     * ===========================
     */

    Actions::<Cache>::set_status_cache(env, LifecycleState::Stopping)
        .await
        .unwrap();

    Actions::<Cache>::remove_actions_cache(env).await.unwrap();

    Actions::<Cache>::set_status_cache(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = Actions::<Cache>::get_actions_cache(env).await;
    assert!(final_cache.is_some_and(|val| val.models.is_empty()));
}
