use std::collections::{HashMap, HashSet};

use application::{handler::SubscribedIndicators, utils::Cache};
use models::{entities::indicators::Model, enums::LifecycleState, structs::Environments};

fn mock_indicator(symbol: &str, strategy_id: i32) -> Model {
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

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    // Reset cache
    SubscribedIndicators::<Cache>::reset_subscribed_indicators_cache(env)
        .await
        .unwrap();

    // Status off
    let status = SubscribedIndicators::new(env)
        .get_subscribed_indicators_state()
        .await;
    assert_eq!(status, LifecycleState::Off);

    let cache = SubscribedIndicators::<Cache>::get_subscribed_indicators_cache(env).await;
    assert!(cache.is_none() || cache.unwrap().is_empty());

    /* ===========================
     * STARTING
     * ===========================
     */

    SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Starting)
        .await
        .unwrap();

    let starting_status = SubscribedIndicators::new(env)
        .get_subscribed_indicators_state()
        .await;
    assert_eq!(starting_status, LifecycleState::Starting);

    /* ===========================
     * LOAD MULTIPLE INDICATORS
     * ===========================
     */

    let indicators = vec![
        mock_indicator("BTCUSDT", 1),
        mock_indicator("BTCUSDT", 2),
        mock_indicator("ETHUSDT", 3),
    ];

    let mut map: HashMap<String, HashSet<i32>> = HashMap::new();
    for ind in &indicators {
        map.entry(ind.symbol.clone())
            .or_insert_with(HashSet::new)
            .insert(ind.strategy_id);
    }

    SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Running)
        .await
        .unwrap();

    SubscribedIndicators::<Cache>::set_subscribed_indicators_cache(env, map)
        .await
        .unwrap();

    let cache = SubscribedIndicators::<Cache>::get_subscribed_indicators_cache(env)
        .await
        .unwrap();
    assert_eq!(cache.len(), 2);
    assert!(cache.get("BTCUSDT").unwrap().contains(&1));
    assert!(cache.get("BTCUSDT").unwrap().contains(&2));
    assert!(cache.get("ETHUSDT").unwrap().contains(&3));

    /* ===========================
     * INSERT INDIVIDUAL INDICATOR
     * ===========================
     */

    let extra = mock_indicator("BTCUSDT", 99);
    SubscribedIndicators::<Cache>::upsert_subscribed_indicator_cache(env, extra.clone())
        .await
        .unwrap();

    let btc = SubscribedIndicators::<Cache>::get_subscribed_indicator_cache(env, "BTCUSDT")
        .await
        .unwrap();
    assert!(btc.contains(&99));

    /* ===========================
     * STOP
     * ===========================
     */

    SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Stopping)
        .await
        .unwrap();

    SubscribedIndicators::<Cache>::remove_all_subscribed_indicators_cache(env)
        .await
        .unwrap();

    SubscribedIndicators::<Cache>::set_status_cache(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = SubscribedIndicators::<Cache>::get_subscribed_indicators_cache(env).await;
    assert!(final_cache.is_some_and(|val| val.is_empty()));

    let final_status = SubscribedIndicators::new(env)
        .get_subscribed_indicators_state()
        .await;
    assert_eq!(final_status, LifecycleState::Off);
}
