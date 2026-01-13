use application::{handler::Strategies, utils::Cache};
use models::{entities::strategies::Model, enums::LifecycleState, structs::Environments};

fn mock_strategy(id: i32) -> Model {
    Model {
        id,
        is_active: true,
        ..Default::default()
    }
}

#[tokio::test]
async fn full_strategies_cache_flow_should_work_correctly() {
    let env = Environments::DEV;

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    let reset = Strategies::<Cache>::reset_strategies_cache(env).await;
    assert!(reset.is_ok());

    // Cannot set Off twice
    let off = Strategies::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    assert!(off.is_err());

    let status = Strategies::default()
        .with_env(env)
        .get_strategies_state()
        .await;
    assert_eq!(status, LifecycleState::Off);

    let cache = Strategies::<Cache>::get_strategies_cache(env).await;
    assert!(cache.is_some_and(|val| val.models.is_empty()));

    /* ===========================
     * LOAD MULTIPLE STRATEGIES
     * ===========================
     */

    let starting = Strategies::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
    assert!(starting.is_ok());

    let strategies = vec![mock_strategy(1), mock_strategy(2)];

    let running = Strategies::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    assert!(running.is_ok());

    Strategies::<Cache>::set_strategies_cache(env, strategies)
        .await
        .unwrap();

    let cache = Strategies::<Cache>::get_strategies_cache(env)
        .await
        .unwrap();
    assert_eq!(cache.models.len(), 2);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * INSERT / UPDATE STRATEGY
     * ===========================
     */

    let extra = mock_strategy(3);
    Strategies::<Cache>::upsert_strategy_cache(env, extra.clone())
        .await
        .unwrap();

    let single = Strategies::<Cache>::get_strategy_cache(env, 3)
        .await
        .unwrap();
    assert_eq!(single.model, extra);
    assert!(single.last_error_message.is_none());
    assert!(single.last_error_date.is_none());

    /* ===========================
     * SET ERROR STATE
     * ===========================
     */

    Strategies::<Cache>::set_strategy_error_cache(env, 3, Some("boom".to_string()))
        .await
        .unwrap();

    let errored = Strategies::<Cache>::get_strategy_cache(env, 3)
        .await
        .unwrap();
    assert_eq!(errored.last_error_message, Some("boom".to_string()));
    assert!(errored.last_error_date.is_some());

    /* ===========================
     * POSTING LIFECYCLE
     * ===========================
     */

    assert!(
        Strategies::<Cache>::set_strategy_posting_cache(env, 3, true)
            .await
            .is_ok()
    );

    // Cannot post twice
    assert!(
        Strategies::<Cache>::set_strategy_posting_cache(env, 3, true)
            .await
            .is_err()
    );

    assert!(
        Strategies::<Cache>::set_strategy_posting_cache(env, 3, false)
            .await
            .is_ok()
    );

    /* ===========================
     * STOP
     * ===========================
     */

    Strategies::<Cache>::set_status_cache(env, LifecycleState::Stopping)
        .await
        .unwrap();

    Strategies::<Cache>::remove_strategies_cache(env)
        .await
        .unwrap();

    Strategies::<Cache>::set_status_cache(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = Strategies::<Cache>::get_strategies_cache(env).await;
    assert!(final_cache.is_some_and(|val| val.models.is_empty()));
}
