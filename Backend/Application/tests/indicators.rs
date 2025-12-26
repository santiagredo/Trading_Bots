use application::{handler::Indicators, utils::Cache};
use models::{entities::indicators::Model, structs::Environments};

fn mock_indicator(strategy_id: i32) -> Model {
    Model {
        id: strategy_id,
        strategy_id,
        is_active: true,
        ..Default::default()
    }
}

#[tokio::test]
async fn full_indicators_cache_flow_should_work_correctly() {
    let env = Environments::DEV;

    // Initial state
    Indicators::<Cache>::stop_active_indicators_cache(&env).await;
    assert!(!Indicators::<Cache>::get_active_indicators_status_cache(&env).await);

    // Load multiple indicators
    let indicators = vec![mock_indicator(1), mock_indicator(2)];
    Indicators::<Cache>::set_active_indicators_cache(&env, indicators).await;

    let cache = Indicators::<Cache>::get_active_indicators_cache(&env)
        .await
        .unwrap();
    assert_eq!(cache.len(), 2);

    // Insert individual indicator
    let extra = mock_indicator(3);
    Indicators::<Cache>::set_active_indicator_cache(&env, extra.clone(), false).await;

    let single = Indicators::<Cache>::get_active_indicator_cache(&env, &3).await;
    assert_eq!(single, Some(extra));

    // Status is true
    assert!(Indicators::<Cache>::get_active_indicators_status_cache(&env).await);

    // Stop
    Indicators::<Cache>::stop_active_indicators_cache(&env).await;

    let final_cache = Indicators::<Cache>::get_active_indicators_cache(&env)
        .await
        .unwrap();

    assert!(final_cache.is_empty());
    assert!(!Indicators::<Cache>::get_active_indicators_status_cache(&env).await);
}
