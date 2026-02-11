use application::handler::SubscribedIndicators;

fn mock_symbols() -> Vec<String> {
    vec![
        "BTCUSDT".to_string(),
        "BTCUSDT".to_string(),
        "ETHUSDT".to_string(),
    ]
}

#[tokio::test]
async fn full_subscribed_indicators_cache_flow_should_work_correctly() {
    let service = SubscribedIndicators::blank();
    service.reset().await.unwrap();

    service.set_all(mock_symbols()).await.unwrap();

    let cache = service.get_all().await;

    assert_eq!(cache.models.len(), 2);

    assert_eq!(cache.models.get("BTCUSDT"), Some(&2));

    assert_eq!(cache.models.get("ETHUSDT"), Some(&1));

    /* ===========================
     * UPSERT INDIVIDUAL INDICATOR
     * ===========================
     */

    service.upsert("BTCUSDT".to_string()).await.unwrap();
    service.upsert("BTCUSDT".to_string()).await.unwrap();

    let btc = service.get("BTCUSDT").await.unwrap();
    assert_eq!(btc, 4);
}
