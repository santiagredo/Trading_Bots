use application::handler::Tasks;
use models::{
    entities::tasks::Model,
    enums::{LifecycleState, TaskState},
    structs::Environments,
};

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

    let reset = Tasks::reset_tasks_cache(env).await;
    assert!(reset.is_ok());

    let off = Tasks::set_status_cache(env, LifecycleState::Off).await;
    assert!(off.is_err());

    let status = Tasks::new().get_tasks_state(env).await;
    assert_eq!(status, LifecycleState::Off);

    let cache = Tasks::get_tasks_cache(env).await;
    assert!(cache.is_some_and(|val| val.models.is_empty()));

    /* ===========================
     * LOAD MULTIPLE TASKS
     * ===========================
     */

    Tasks::set_status_cache(env, LifecycleState::Starting)
        .await
        .unwrap();

    Tasks::set_status_cache(env, LifecycleState::Running)
        .await
        .unwrap();

    let tasks = vec![mock_task(1, "BNUAB"), mock_task(2, "BNUEI")];
    Tasks::set_tasks_cache(env, tasks).await.unwrap();

    let cache = Tasks::get_tasks_cache(env).await.unwrap();
    assert_eq!(cache.models.len(), 2);
    assert_eq!(cache.status, LifecycleState::Running);

    for task in cache.models.values() {
        assert_eq!(task.state, TaskState::Sleeping);
    }

    /* ===========================
     * INSERT INDIVIDUAL TASK
     * ===========================
     */

    let extra = mock_task(3, "CPUPS");
    Tasks::upsert_task_cache(env, extra.clone()).await.unwrap();

    let single = Tasks::get_task_cache(env, 3).await.unwrap();
    assert_eq!(single.model, extra);
    assert_eq!(single.state, TaskState::Sleeping);

    /* ===========================
     * REMOVE TASK
     * ===========================
     */

    let removed = Tasks::remove_task_cache(env, 3).await.unwrap().unwrap();

    assert_eq!(removed.model, extra);

    let not_found = Tasks::get_task_cache(env, 3).await;
    assert!(not_found.is_none());

    /* ===========================
     * STOP
     * ===========================
     */

    Tasks::set_status_cache(env, LifecycleState::Stopping)
        .await
        .unwrap();

    Tasks::remove_tasks_cache(env).await.unwrap();

    Tasks::set_status_cache(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = Tasks::get_tasks_cache(env).await;
    assert!(final_cache.is_some_and(|val| val.models.is_empty()));
}
