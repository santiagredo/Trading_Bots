use application::handler::SubscribedIndicators;
use models::{entities::indicators::Model, enums::LifecycleState, structs::Environments};
use std::collections::{HashMap, HashSet};

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
    let service = SubscribedIndicators::blank();

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    service.reset(env).await.unwrap();

    let status = service.state(env).await;
    assert_eq!(status, LifecycleState::Off);

    let cache = service.get_all(env).await;
    assert!(cache.is_none());

    /* ===========================
     * STARTING
     * ===========================
     */

    service
        .set_state(env, LifecycleState::Starting)
        .await
        .unwrap();

    let starting_status = service.state(env).await;
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

    let mut grouped: HashMap<String, HashSet<i32>> = HashMap::new();
    for ind in indicators {
        grouped
            .entry(ind.symbol)
            .or_insert_with(HashSet::new)
            .insert(ind.strategy_id);
    }

    let values: Vec<(String, Vec<i32>)> = grouped
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().collect()))
        .collect();

    service
        .set_state(env, LifecycleState::Running)
        .await
        .unwrap();

    service.set_all(env, values).await.unwrap();

    let cache = service.get_all(env).await.unwrap();
    assert_eq!(cache.models.len(), 2);
    assert!(cache.models.get("BTCUSDT").unwrap().contains(&1));
    assert!(cache.models.get("BTCUSDT").unwrap().contains(&2));
    assert!(cache.models.get("ETHUSDT").unwrap().contains(&3));

    /* ===========================
     * UPSERT INDIVIDUAL INDICATOR
     * ===========================
     */

    service
        .upsert(env, "BTCUSDT".into(), vec![99])
        .await
        .unwrap();

    let btc = service.get(env, "BTCUSDT".into()).await.unwrap();
    assert!(btc.contains(&99));

    /* ===========================
     * STOP
     * ===========================
     */

    service
        .set_state(env, LifecycleState::Stopping)
        .await
        .unwrap();

    service.remove_all(env).await.unwrap();

    service.set_state(env, LifecycleState::Off).await.unwrap();

    let final_cache = service.get_all(env).await;
    assert!(final_cache.is_none());

    let final_status = service.state(env).await;
    assert_eq!(final_status, LifecycleState::Off);
}
