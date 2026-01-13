use application::{handler::Tasks, utils::Cache};
use models::{entities::tasks::Model, enums::LifecycleState, structs::Environments};

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

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    let reset = Tasks::<Cache>::reset_tasks_cache(env).await;
    assert!(reset.is_ok());

    let off = Tasks::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    assert!(off.is_err());

    let status = Tasks::default().with_env(env).get_tasks_state().await;
    assert_eq!(status, LifecycleState::Off);

    let cache = Tasks::<Cache>::get_tasks_cache(env).await;
    assert!(cache.is_some_and(|val| val.models.is_empty()));

    /* ===========================
     * LOAD MULTIPLE TASKS
     * ===========================
     */

    let starting = Tasks::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
    assert!(starting.is_ok());

    let tasks = vec![mock_task(1, "BNUAB"), mock_task(2, "BNUEI")];

    let running = Tasks::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    assert!(running.is_ok());

    Tasks::<Cache>::set_tasks_cache(env, tasks).await.unwrap();

    let cache = Tasks::<Cache>::get_tasks_cache(env).await.unwrap();
    assert_eq!(cache.models.len(), 2);
    assert_eq!(cache.status, LifecycleState::Running);

    /* ===========================
     * INSERT INDIVIDUAL TASK
     * ===========================
     */

    let extra = mock_task(3, "CPUPS");
    Tasks::<Cache>::upsert_task_cache(env, extra.clone())
        .await
        .unwrap();

    let single = Tasks::<Cache>::get_task_cache(env, 3).await.unwrap();
    assert_eq!(single, extra);

    /* ===========================
     * REMOVE TASK
     * ===========================
     */

    let removed = Tasks::<Cache>::remove_task_cache(env, 3).await.unwrap();

    assert_eq!(removed, Some(extra));

    let not_found = Tasks::<Cache>::get_task_cache(env, 3).await;
    assert!(not_found.is_none());

    /* ===========================
     * STOP
     * ===========================
     */

    Tasks::<Cache>::set_status_cache(env, LifecycleState::Stopping)
        .await
        .unwrap();

    Tasks::<Cache>::remove_tasks_cache(env).await.unwrap();

    Tasks::<Cache>::set_status_cache(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = Tasks::<Cache>::get_tasks_cache(env).await;
    assert!(final_cache.is_some_and(|val| val.models.is_empty()));
}
