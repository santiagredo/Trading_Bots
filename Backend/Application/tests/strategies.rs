use application::{handler::Strategies, utils::Cache};
use models::{entities::strategies::Model, enums::LifecycleState, structs::Environments};

#[tokio::test]
async fn full_strategies_cache_flow_should_work_correctly() {
    fn mock_strategy(id: i32) -> Model {
        Model {
            id,
            is_active: true,
            ..Default::default()
        }
    }

    let env = Environments::DEV;

    // Reset (safe even if env doesn't exist)
    Strategies::<Cache>::reset_strategies_cache(env)
        .await
        .unwrap();

    // Env not created yet → Off by default
    assert_eq!(
        Strategies::<Cache>::get_strategies_state_cache(env).await,
        LifecycleState::Off
    );

    let cache = Strategies::<Cache>::get_strategies_cache(env)
        .await
        .unwrap();
    assert!(cache.models.is_empty());

    // Start lifecycle
    Strategies::<Cache>::set_status_cache(env, LifecycleState::Starting)
        .await
        .unwrap();

    Strategies::<Cache>::set_status_cache(env, LifecycleState::Running)
        .await
        .unwrap();

    // Load strategies
    let strategies = vec![mock_strategy(1), mock_strategy(2)];
    Strategies::<Cache>::set_strategies_cache(env, strategies)
        .await
        .unwrap();

    let cache = Strategies::<Cache>::get_strategies_cache(env)
        .await
        .unwrap();
    assert_eq!(cache.models.len(), 2);

    // Upsert
    let extra = mock_strategy(3);
    Strategies::<Cache>::upsert_strategy_cache(env, extra.clone())
        .await
        .unwrap();

    let single = Strategies::<Cache>::get_strategy_cache(env, 3)
        .await
        .unwrap();
    assert_eq!(single.model, extra);

    // Error state
    Strategies::<Cache>::set_strategy_error_cache(env, 3, Some("boom".into()))
        .await
        .unwrap();

    let errored = Strategies::<Cache>::get_strategy_cache(env, 3)
        .await
        .unwrap();
    assert_eq!(errored.last_error_message, Some("boom".into()));

    // Stop
    Strategies::<Cache>::set_status_cache(env, LifecycleState::Stopping)
        .await
        .unwrap();

    let removed = Strategies::<Cache>::remove_strategies_cache(env)
        .await
        .unwrap();
    assert_eq!(removed.len(), 3);

    Strategies::<Cache>::set_status_cache(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = Strategies::<Cache>::get_strategies_cache(env)
        .await
        .unwrap();
    assert!(final_cache.models.is_empty());
}
