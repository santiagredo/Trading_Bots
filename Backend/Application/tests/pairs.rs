use application::{handler::Pairs, utils::Cache};
use models::{entities::pairs::Model, structs::Environments};

fn mock_pair(id: i32) -> Model {
    Model {
        id,
        ..Default::default()
    }
}

#[tokio::test]
async fn full_pairs_cache_flow_should_work_correctly() {
    let env = Environments::DEV;

    // Initial state
    Pairs::<Cache>::stop_active_pairs_cache(&env).await;
    assert!(!Pairs::<Cache>::get_active_pairs_status_cache(&env).await);

    // Load multiple pairs
    let pairs = vec![mock_pair(1), mock_pair(2)];
    Pairs::<Cache>::set_active_pairs_cache(&env, pairs).await;

    let cache = Pairs::<Cache>::get_active_pairs_cache(&env).await.unwrap();
    assert_eq!(cache.len(), 2);

    // Insert individual pair
    let extra = mock_pair(3);
    Pairs::<Cache>::set_active_pair_cache(&env, extra.clone(), false).await;

    let single = Pairs::<Cache>::get_active_pair_cache(&env, &3).await;
    assert_eq!(single, Some(extra));

    // Status is true
    assert!(Pairs::<Cache>::get_active_pairs_status_cache(&env).await);

    // Stop
    Pairs::<Cache>::stop_active_pairs_cache(&env).await;

    let final_cache = Pairs::<Cache>::get_active_pairs_cache(&env).await.unwrap();

    assert!(final_cache.is_empty());
    assert!(!Pairs::<Cache>::get_active_pairs_status_cache(&env).await);
}
