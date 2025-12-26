use application::{handler::Strategies, utils::Cache};
use models::{entities::strategies::Model, structs::Environments};

fn mock_strategy(id: i32) -> Model {
    Model {
        id,
        ..Default::default()
    }
}

#[tokio::test]
async fn full_strategies_cache_flow_should_work_correctly() {
    let env = Environments::DEV;

    // Initial state
    Strategies::<Cache>::stop_active_strategies_cache(&env).await;
    assert!(!Strategies::<Cache>::get_active_strategies_status_cache(&env).await);

    // Load multiple strategies
    let strategies = vec![mock_strategy(1), mock_strategy(2)];
    Strategies::<Cache>::set_active_strategies_cache(&env, strategies).await;

    let cache = Strategies::<Cache>::get_active_strategies_cache(&env)
        .await
        .unwrap();
    assert_eq!(cache.len(), 2);

    // Insert/update with error
    let updated = mock_strategy(3);
    Strategies::<Cache>::set_active_strategy_cache(
        &env,
        updated.clone(),
        false,
        Some("boom".to_string()),
    )
    .await;

    let cached = Strategies::<Cache>::get_active_strategy_cache(&env, &3)
        .await
        .unwrap();

    assert_eq!(cached.model, updated);
    assert!(cached.last_error_date.is_some());
    assert_eq!(cached.last_error_message, Some("boom".to_string()));

    // Posting lifecycle
    assert!(
        Strategies::<Cache>::set_active_strategy_posting_cache(env, 3, true)
            .await
            .is_ok()
    );

    assert!(
        Strategies::<Cache>::set_active_strategy_posting_cache(env, 3, true)
            .await
            .is_err()
    );

    assert!(
        Strategies::<Cache>::set_active_strategy_posting_cache(env, 3, false)
            .await
            .is_ok()
    );

    // Status is true
    assert!(Strategies::<Cache>::get_active_strategies_status_cache(&env).await);

    // Stop
    Strategies::<Cache>::stop_active_strategies_cache(&env).await;

    let final_cache = Strategies::<Cache>::get_active_strategies_cache(&env)
        .await
        .unwrap();

    assert!(final_cache.is_empty());
    assert!(!Strategies::<Cache>::get_active_strategies_status_cache(&env).await);
}
