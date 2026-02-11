use application::{handler::OrderStatus, utils::EntityCache};
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

    let reset = OrderStatus::blank().reset(env).await;
    assert!(reset.is_ok());

    // Cannot transition to Off explicitly from reset
    let off = OrderStatus::blank()
        .set_state(env, LifecycleState::Off)
        .await;
    assert!(off.is_err());

    let state = OrderStatus::blank().state(env).await;
    assert_eq!(state, LifecycleState::Off);

    let cache = OrderStatus::blank().get_all(env).await;
    assert!(cache.as_ref().is_none(), "{cache:?}");

    /* ===========================
     * LOAD MULTIPLE STATUSES
     * ===========================
     */

    let starting = OrderStatus::blank()
        .set_state(env, LifecycleState::Starting)
        .await;
    assert!(starting.is_ok());

    let statuses = vec![
        mock_status(1, "OPEN"),
        mock_status(2, "COMPLETED"),
        mock_status(3, "ABORTED"),
    ];

    let running = OrderStatus::blank()
        .set_state(env, LifecycleState::Running)
        .await;
    assert!(running.is_ok());

    OrderStatus::blank().set_all(env, statuses).await.unwrap();

    let cache = OrderStatus::blank().get_all(env).await.unwrap();
    assert_eq!(cache.models.len(), 3);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * GET SINGLE STATUS
     * ===========================
     */

    let completed = OrderStatus::blank()
        .get_by_status(env, Status::Completed)
        .await
        .unwrap();

    assert_eq!(completed.id, 2);

    /* ===========================
     * STOP
     * ===========================
     */

    OrderStatus::blank()
        .set_state(env, LifecycleState::Stopping)
        .await
        .unwrap();

    OrderStatus::blank().remove_all(env).await.unwrap();

    OrderStatus::blank()
        .set_state(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = OrderStatus::blank().get_all(env).await;
    assert!(final_cache.as_ref().is_none(), "{final_cache:?}");
}
