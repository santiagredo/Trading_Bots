use application::{handler::Tasks, utils::EntityCache};
use models::{
    entities::tasks::Model,
    enums::{LifecycleState, TaskState},
    structs::{CacheTask, Environments},
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

    let reset = Tasks::blank().reset(env).await;
    assert!(reset.is_ok());

    let off = Tasks::blank().set_state(env, LifecycleState::Off).await;
    assert!(off.is_err());

    let status = Tasks::blank().state(env).await;
    assert_eq!(status, LifecycleState::Off);

    let cache = Tasks::blank().get_all(env).await;
    assert!(cache.is_none(), "{cache:?}");

    /* ===========================
     * LOAD MULTIPLE TASKS
     * ===========================
     */

    Tasks::blank()
        .set_state(env, LifecycleState::Starting)
        .await
        .unwrap();

    Tasks::blank()
        .set_state(env, LifecycleState::Running)
        .await
        .unwrap();

    let tasks = vec![mock_task(1, "BNUAB"), mock_task(2, "BNUEI")]
        .into_iter()
        .map(|task| CacheTask {
            model: task,
            state: TaskState::Sleeping,
        })
        .collect();
    Tasks::blank().set_all(env, tasks).await.unwrap();

    let cache = Tasks::blank().get_all(env).await.unwrap();
    assert_eq!(cache.models.len(), 2);
    assert_eq!(cache.status, LifecycleState::Running);

    for task in cache.models.values() {
        assert_eq!(task.state, TaskState::Sleeping);
    }

    /* ===========================
     * INSERT INDIVIDUAL TASK
     * ===========================
     */

    let extra = CacheTask {
        model: mock_task(3, "CPUPS"),
        state: TaskState::Sleeping,
    };
    Tasks::blank()
        .upsert(env, extra.model.id, extra.clone())
        .await
        .unwrap();

    let single = Tasks::blank().get(env, 3).await.unwrap();
    assert_eq!(single.model, extra.model);
    assert_eq!(single.state, TaskState::Sleeping);

    /* ===========================
     * REMOVE TASK
     * ===========================
     */

    let removed = Tasks::blank().remove(env, 3).await.unwrap().unwrap();

    assert_eq!(removed.model, extra.model);

    let not_found = Tasks::blank().get(env, 3).await;
    assert!(not_found.is_none());

    /* ===========================
     * STOP
     * ===========================
     */

    Tasks::blank()
        .set_state(env, LifecycleState::Stopping)
        .await
        .unwrap();

    Tasks::blank().remove_all(env).await.unwrap();

    Tasks::blank()
        .set_state(env, LifecycleState::Off)
        .await
        .unwrap();

    let final_cache = Tasks::blank().get_all(env).await;
    assert!(final_cache.is_none(), "{final_cache:?}");
}
