use application::{handler::Actions, utils::Cache};
use models::{entities::actions::Model, structs::Environments};
use sea_orm::prelude::Decimal;

fn mock_model(strategy_id: i32) -> Model {
    Model {
        id: strategy_id,
        strategy_id,
        is_active: true,
        is_sell: false,
        is_quote_asset: false,
        is_percentage: false,
        value: Decimal::new(50, 0),
        pair_id: 1,
    }
}

#[tokio::test]
async fn full_cache_flow_should_work_correctly() {
    let env = Environments::DEV;

    // Initial state
    Actions::<Cache>::stop_active_actions_cache(&env).await;
    assert!(!Actions::<Cache>::get_active_actions_status_cache(&env).await);

    // Load multiple models
    let models = vec![mock_model(1), mock_model(2)];
    Actions::<Cache>::set_active_actions_cache(&env, models.clone()).await;

    let cache = Actions::<Cache>::get_active_actions_cache(&env)
        .await
        .unwrap();
    assert_eq!(cache.len(), 2);

    // Insert individual model 
    let extra = mock_model(3);
    Actions::<Cache>::set_active_action_cache(&env, extra.clone(), false).await;

    let single = Actions::<Cache>::get_active_action_cache(&env, &3).await;
    assert_eq!(single, Some(extra));

    // Status is true
    assert!(Actions::<Cache>::get_active_actions_status_cache(&env).await);

    // Stop
    Actions::<Cache>::stop_active_actions_cache(&env).await;

    let final_cache = Actions::<Cache>::get_active_actions_cache(&env)
        .await
        .unwrap();

    assert!(final_cache.is_empty());
    assert!(!Actions::<Cache>::get_active_actions_status_cache(&env).await);
}
