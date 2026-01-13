use application::{handler::OrderStatus, utils::Cache};
use models::{
    entities::status::Model,
    enums::{LifecycleState, Status},
    structs::Environments,
};

fn mock_status(id: i32, name: &str) -> Model {
    Model {
        id,
        name: name.to_string(),
    }
}

#[tokio::test]
async fn full_order_status_cache_flow_should_work_correctly() {
    let env = Environments::DEV;

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    let reset = OrderStatus::<Cache>::reset_status_cache(env).await;
    assert!(reset.is_ok());

    // Cannot transition to Off explicitly from reset
    let off = OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    assert!(off.is_err());

    let state = OrderStatus::<Cache>::get_cache_state(env).await;
    assert_eq!(state, LifecycleState::Off);

    let cache = OrderStatus::<Cache>::get_status_cache(env).await;
    assert!(cache.is_some_and(|val| val.models.is_empty()));

    /* ===========================
     * LOAD MULTIPLE STATUSES
     * ===========================
     */

    let starting = OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
    assert!(starting.is_ok());

    let statuses = vec![
        mock_status(1, "OPEN"),
        mock_status(2, "COMPLETED"),
        mock_status(3, "ABORTED"),
    ];

    let running = OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    assert!(running.is_ok());

    OrderStatus::<Cache>::set_statuses_cache(env, statuses)
        .await
        .unwrap();

    let cache = OrderStatus::<Cache>::get_status_cache(env).await.unwrap();
    assert_eq!(cache.models.len(), 3);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * GET SINGLE STATUS
     * ===========================
     */

    let completed = OrderStatus::<Cache>::get_status_by_enum(env, Status::Completed)
        .await
        .unwrap();

    assert_eq!(completed.id, 2);

    /* ===========================
     * STOP
     * ===========================
     */

    OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Stopping)
        .await
        .unwrap();

    OrderStatus::<Cache>::remove_statuses_cache(env)
        .await
        .unwrap();

    OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = OrderStatus::<Cache>::get_status_cache(env).await;
    assert!(final_cache.is_some_and(|val| val.models.is_empty()));
}
