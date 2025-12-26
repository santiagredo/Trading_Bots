use application::{handler::Tasks, utils::Cache};
use models::{entities::tasks::Model, structs::Environments};

fn mock_task(id: i32, nick: &str) -> Model {
    Model {
        id,
        nick: nick.to_string(),
        delay: 10000,
        cooldown: 10000,
        is_active: true,
        ..Default::default()
    }
}

#[tokio::test]
async fn full_tasks_cache_flow_should_work_correctly() {
    let env = Environments::DEV;

    // Initial
    Tasks::<Cache>::stop_active_tasks_cache(&env).await;
    assert!(!Tasks::<Cache>::get_active_tasks_status_cache(&env).await);

    // Bulk load
    let tasks = vec![mock_task(1, "BNUAB"), mock_task(2, "BNUEI")];

    Tasks::<Cache>::set_active_tasks_cache(&env, tasks).await;

    let cached = Tasks::<Cache>::get_active_tasks_cache(&env).await.unwrap();

    assert_eq!(cached.len(), 2);
    assert!(Tasks::<Cache>::get_active_tasks_status_cache(&env).await);

    // Insert single
    let extra = mock_task(3, "CPUPS");
    Tasks::<Cache>::set_active_task_cache(&env, extra.clone(), false).await;

    let single = Tasks::<Cache>::get_active_task_cache(&env, &3).await;
    assert_eq!(single, Some(extra));

    // Stop
    Tasks::<Cache>::stop_active_tasks_cache(&env).await;

    let final_tasks = Tasks::<Cache>::get_active_tasks_cache(&env)
        .await
        .unwrap_or_default();

    assert!(final_tasks.is_empty());
    assert!(!Tasks::<Cache>::get_active_tasks_status_cache(&env).await);
}
