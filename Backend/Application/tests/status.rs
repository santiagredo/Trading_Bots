use application::{handler::OrderStatus, utils::Cache};
use models::{entities::status::Model, enums::Status, structs::Environments};

fn mock_model(id: i32, name: &str) -> Model {
    Model {
        id,
        name: name.to_string(),
    }
}

#[tokio::test]
async fn full_order_status_cache_flow_should_work_correctly() {
    let env = Environments::DEV;

    // Initial state
    OrderStatus::<Cache>::stop_active_status_cache(&env).await;
    assert!(!OrderStatus::<Cache>::get_active_status_status_cache(&env).await);

    // Load multiple statuses
    let models = vec![
        mock_model(1, "OPEN"),
        mock_model(2, "COMPLETED"),
        mock_model(3, "ABORTED"),
    ];

    OrderStatus::<Cache>::set_active_status_cache(&env, models).await;

    let cache = OrderStatus::<Cache>::get_active_status_cache(&env)
        .await
        .unwrap();

    assert_eq!(cache.len(), 3);

    // Get single status
    let completed = OrderStatus::<Cache>::get_active_status(&env, &Status::Completed).await;
    assert_eq!(completed, Some(2));

    // Status flag
    assert!(OrderStatus::<Cache>::get_active_status_status_cache(&env).await);

    // Stop
    OrderStatus::<Cache>::stop_active_status_cache(&env).await;

    let final_cache = OrderStatus::<Cache>::get_active_status_cache(&env)
        .await
        .unwrap();

    assert!(final_cache.is_empty());
    assert!(!OrderStatus::<Cache>::get_active_status_status_cache(&env).await);
}
