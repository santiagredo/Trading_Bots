use application::{handler::Strategies, utils::EntityCache};
use models::{
    entities::strategies::Model,
    enums::LifecycleState,
    structs::{CacheStrategy, Environments},
};

fn mock_strategy(id: i32) -> Model {
    Model {
        id,
        is_active: true,
        ..Default::default()
    }
}

#[tokio::test]
async fn full_strategies_flow_should_work_correctly() {
    let env = Environments::DEV;
    let service = Strategies::blank();

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    service.reset_strategies(env).await.unwrap();

    let status = service.state(env).await;
    assert_eq!(status, LifecycleState::Off);

    let cache = service.get_all(env).await;
    assert!(cache.is_none());

    /* ===========================
     * START STRATEGIES
     * ===========================
     */

    service
        .set_state(env, LifecycleState::Starting)
        .await
        .unwrap();

    service
        .set_state(env, LifecycleState::Running)
        .await
        .unwrap();

    let cache = service.get_all(env).await.unwrap();
    assert_eq!(cache.status, LifecycleState::Running);
    assert!(cache.models.is_empty());

    /* ===========================
     * LOAD MULTIPLE STRATEGIES
     * ===========================
     */

    let strategies = vec![mock_strategy(1), mock_strategy(2)]
        .into_iter()
        .map(|val| CacheStrategy {
            model: val,
            ..Default::default()
        })
        .collect();

    service.set_all(env, strategies).await.unwrap();

    let cache = service.get_all(env).await.unwrap();
    assert_eq!(cache.models.len(), 2);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * INSERT INDIVIDUAL STRATEGY
     * ===========================
     */

    let extra = mock_strategy(3);
    service
        .upsert(
            env,
            extra.id,
            CacheStrategy {
                model: extra.clone(),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let single = service.get(env, 3).await.unwrap();
    assert_eq!(single.model, extra);

    /* ===========================
     * ERROR STATE
     * ===========================
     */

    service
        .set_strategy_error(env, 3, Some("boom".into()))
        .await
        .unwrap();

    let errored = service.get(env, 3).await.unwrap();
    assert_eq!(errored.last_error_message, Some("boom".into()));

    /* ===========================
     * STOP
     * ===========================
     */

    service
        .set_state(env, LifecycleState::Stopping)
        .await
        .unwrap();

    let removed = service.remove_all(env).await.unwrap();
    assert_eq!(removed.len(), 3);

    service.set_state(env, LifecycleState::Off).await.unwrap();

    let final_result = service.get_all(env).await;
    assert!(final_result.is_none());
}
