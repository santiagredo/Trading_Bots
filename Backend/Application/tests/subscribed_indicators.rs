use std::collections::{HashMap, HashSet};

use application::{handler::SubscribedIndicators, utils::Cache};
use models::{entities::indicators::Model, structs::Environments};

fn mock_model(symbol: &str, strategy_id: i32) -> Model {
    Model {
        id: strategy_id,
        strategy_id,
        symbol: symbol.to_string(),
        ..Default::default()
    }
}

#[tokio::test]
async fn full_subscribed_indicators_cache_flow_should_work_correctly() {
    let env = Environments::DEV;

    // Initial state
    SubscribedIndicators::<Cache>::stop_active_subscribed_indicators_cache(&env).await;
    assert!(
        !SubscribedIndicators::<Cache>::get_active_subscribed_indicators_status_cache(&env).await
    );

    // Bulk load
    let map = HashMap::from([
        ("BTCUSDT".to_string(), HashSet::from([1, 2])),
        ("ETHUSDT".to_string(), HashSet::from([3])),
    ]);

    SubscribedIndicators::<Cache>::set_active_subscribed_indicators_cache(&env, map).await;

    let cache = SubscribedIndicators::<Cache>::get_active_subscribed_indicators_cache(&env)
        .await
        .unwrap();

    assert_eq!(cache.len(), 2);

    // Insert individual indicator
    let extra = mock_model("BTCUSDT", 99);
    SubscribedIndicators::<Cache>::set_active_subscribed_indicator_cache(&env, extra, false).await;

    let btc = SubscribedIndicators::<Cache>::get_active_subscribed_indicator_cache(
        &env,
        "BTCUSDT".to_string(),
    )
    .await
    .unwrap();

    assert!(btc.contains(&99));

    // Status is true
    assert!(
        SubscribedIndicators::<Cache>::get_active_subscribed_indicators_status_cache(&env).await
    );

    // Stop
    SubscribedIndicators::<Cache>::stop_active_subscribed_indicators_cache(&env).await;

    let final_cache = SubscribedIndicators::<Cache>::get_active_subscribed_indicators_cache(&env)
        .await
        .unwrap();

    assert!(final_cache.is_empty());
    assert!(
        !SubscribedIndicators::<Cache>::get_active_subscribed_indicators_status_cache(&env).await
    );
}
