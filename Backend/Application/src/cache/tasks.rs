use std::{collections::HashMap, future::Future, mem, sync::Arc, time::Duration};

use chrono::{Local, NaiveDateTime, Timelike};
use models::{
    entities::tasks::Model,
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState, TaskState},
    structs::{CacheTask, CacheTasks, CacheTasksEnvironments, Environments, TaskRequest},
};
use once_cell::sync::Lazy;
use tokio::{sync::RwLock, time::sleep};
use tokio_util::sync::CancellationToken;

use crate::{
    handler::{Binance, CoinPaprika, Metrics, Tasks},
    utils::Cache,
};

static ACTIVE_TASKS: Lazy<Arc<RwLock<CacheTasksEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheTasksEnvironments::new())));

impl Tasks<Cache> {
    pub async fn set_status_cache(
        environment: Environments,
        status: LifecycleState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache.get_or_create(environment);

        transition_with_timestamp(env_cache, status)?;
        Ok(())
    }

    pub async fn set_tasks_cache(env: Environments, tasks: Vec<Model>) -> Result<(), String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err(format!(
                "Cannot load tasks: cache not running ({:?})",
                env_cache.status
            ));
        }

        env_cache.models = tasks
            .into_iter()
            .map(|t| {
                (
                    t.id,
                    CacheTask {
                        model: t,
                        ..Default::default()
                    },
                )
            })
            .collect();

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    pub async fn set_task_state_cache(
        environment: Environments,
        task_id: i32,
        state: TaskState,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let task = env_cache.models.get_mut(&task_id).ok_or("Task not found")?;

        task.state = state;
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn set_task_last_execution_cache(
        environment: Environments,
        task_id: i32,
        last_execution: NaiveDateTime,
    ) -> Result<(), String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let task = env_cache.models.get_mut(&task_id).ok_or("Task not found")?;

        task.model.last_execution = Some(last_execution);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn upsert_task_cache(environment: Environments, task: Model) -> Result<(), String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache.get_or_create(environment);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(
            task.id,
            CacheTask {
                model: task,
                ..Default::default()
            },
        );
        env_cache.last_update_date = Local::now().naive_local();

        Ok(())
    }

    pub async fn remove_task_cache(
        environment: Environments,
        task_id: i32,
    ) -> Result<Option<CacheTask>, String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&task_id);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    pub async fn remove_tasks_cache(
        environment: Environments,
    ) -> Result<HashMap<i32, CacheTask>, String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache
            .get_mut(&environment)
            .ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.models);
        env_cache.last_update_date = Local::now().naive_local();

        Ok(removed)
    }

    pub async fn get_tasks_cache(environment: Environments) -> Option<CacheTasks> {
        let cache = ACTIVE_TASKS.read().await;
        cache.get(&environment).cloned()
    }

    pub async fn get_task_cache(environment: Environments, strategy_id: i32) -> Option<CacheTask> {
        let cache = ACTIVE_TASKS.read().await;
        cache.get(&environment)?.models.get(&strategy_id).cloned()
    }

    pub async fn get_tasks_state_cache(environment: Environments) -> LifecycleState {
        let cache = ACTIVE_TASKS.read().await;
        cache
            .get(&environment)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    pub async fn reset_tasks_cache(environment: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_TASKS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&environment) {
            *env_cache = CacheTasks::new();
        }

        Ok(())
    }

    fn spawn_task<F, Fut>(
        task: Model,
        environment: Environments,
        cancellation_token: CancellationToken,
        task_fn: F,
    ) where
        F: Fn(Environments) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let delay = task.delay as u64;
        let cooldown = task.cooldown as u64;
        let task_id = task.id;

        tokio::spawn(async move {
            let _ = Self::set_task_state_cache(environment, task_id, TaskState::Sleeping).await;

            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        let _ = Self::set_task_state_cache(
                            environment,
                            task_id,
                            TaskState::Stopped,
                        ).await;
                        break;
                    }
                    _ = sleep(Duration::from_secs(delay)) => {
                        let _ = Self::set_task_state_cache(
                            environment,
                            task_id,
                            TaskState::Running,
                        ).await;

                        task_fn(environment).await;

                        let _ = Self::set_task_state_cache(
                            environment,
                            task_id,
                            TaskState::Saving,
                        ).await;

                        let now = Local::now().naive_local();

                        let _ = Self::set_task_last_execution_cache(environment, task_id, now).await;

                        let task_request = TaskRequest {
                            id: Some(task_id),
                            last_execution: Some(now),
                            ..Default::default()
                        };

                        let _ = Tasks::new(task_request).with_env(environment).update_task().await;

                        let _ = Self::set_task_state_cache(
                            environment,
                            task_id,
                            TaskState::Sleeping,
                        ).await;

                        sleep(Duration::from_secs(cooldown)).await;

                    }
                }
            }
        });
    }

    pub fn run_task(task: Model, environment: Environments, cancellation_token: CancellationToken) {
        match task.nick.as_str() {
            "BNUAB" => Self::spawn_task(task, environment, cancellation_token, |env| async move {
                Binance::update_account_balances(env).await;
            }),
            "BNUEI" => Self::spawn_task(task, environment, cancellation_token, |env| async move {
                Binance::update_exchange_information(env).await;
            }),
            "CPUPS" => Self::spawn_task(task, environment, cancellation_token, |env| async move {
                CoinPaprika::default()
                    .with_env(env)
                    .update_pairs_statistics()
                    .await;
            }),
            "CMPER" => Self::spawn_task(task, environment, cancellation_token, |env| async move {
                let now = Local::now();
                let next_hour = match (now + chrono::Duration::hours(1))
                    .with_minute(0)
                    .and_then(|t| t.with_second(0))
                    .and_then(|t| t.with_nanosecond(0))
                {
                    Some(t) => t,
                    None => {
                        sleep(Duration::from_secs(60)).await;
                        return;
                    }
                };

                let wait = (next_hour - now)
                    .to_std()
                    .unwrap_or(Duration::from_secs(3600));

                sleep(wait).await;

                let _ = Metrics::default().with_env(env).persist_metrics().await;
            }),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use models::{
        entities::tasks::Model,
        enums::{LifecycleState, TaskState},
        structs::Environments,
    };

    use crate::{handler::Tasks, utils::Cache};

    // =========================
    // Helpers
    // =========================

    fn mock_task(id: i32) -> Model {
        Model {
            id,
            is_active: true,
            ..Default::default()
        }
    }

    async fn start_env(env: Environments) {
        let _ = Tasks::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
        let _ = Tasks::<Cache>::set_status_cache(env, LifecycleState::Running).await;
    }

    async fn stop_env(env: Environments) {
        let _ = Tasks::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let _ = Tasks::<Cache>::remove_tasks_cache(env)
            .await
            .expect("remove_tasks_cache should work in Stopping");

        let _ = Tasks::<Cache>::set_status_cache(env, LifecycleState::Off).await;
    }

    // =========================
    // Scenarios (unit responsibilities)
    // =========================

    async fn scenario_reset_env_clears_state(env: Environments) {
        start_env(env).await;

        let tasks = vec![mock_task(1)];
        Tasks::<Cache>::set_tasks_cache(env, tasks).await.unwrap();

        let _ = Tasks::<Cache>::set_status_cache(env, LifecycleState::Stopping).await;

        let removed = Tasks::<Cache>::remove_tasks_cache(env).await.unwrap();
        assert_eq!(removed.len(), 1);

        let cache = Tasks::<Cache>::get_tasks_cache(env).await.unwrap();
        assert!(cache.models.is_empty());

        let _ = Tasks::<Cache>::set_status_cache(env, LifecycleState::Off).await;

        let status = Tasks::<Cache>::get_tasks_state_cache(env).await;
        assert_eq!(status, LifecycleState::Off);
    }

    async fn scenario_set_tasks_cache(env: Environments) {
        start_env(env).await;

        let tasks = vec![mock_task(1), mock_task(2)];
        Tasks::<Cache>::set_tasks_cache(env, tasks).await.unwrap();

        let cache = Tasks::<Cache>::get_tasks_cache(env).await.unwrap();

        assert_eq!(cache.models.len(), 2);
        assert!(cache.models.contains_key(&1));
        assert!(cache.models.contains_key(&2));

        assert_eq!(cache.models.get(&1).unwrap().state, TaskState::Sleeping);
        assert_eq!(cache.models.get(&2).unwrap().state, TaskState::Sleeping);

        let status = Tasks::<Cache>::get_tasks_state_cache(env).await;
        assert_eq!(status, LifecycleState::Running);

        stop_env(env).await;
    }

    async fn scenario_upsert_task_insert(env: Environments) {
        start_env(env).await;

        let task = mock_task(10);
        Tasks::<Cache>::upsert_task_cache(env, task.clone())
            .await
            .unwrap();

        let cached = Tasks::<Cache>::get_task_cache(env, 10).await.unwrap();

        assert_eq!(cached.model.id, task.id);
        assert_eq!(cached.state, TaskState::Sleeping);

        stop_env(env).await;
    }

    async fn scenario_remove_task(env: Environments) {
        start_env(env).await;

        let task = mock_task(20);
        Tasks::<Cache>::upsert_task_cache(env, task).await.unwrap();

        let removed = Tasks::<Cache>::remove_task_cache(env, 20).await.unwrap();
        assert!(removed.is_some());

        let cached = Tasks::<Cache>::get_task_cache(env, 20).await;
        assert!(cached.is_none());

        stop_env(env).await;
    }

    async fn scenario_get_tasks_state_cache(env: Environments) {
        assert_eq!(
            Tasks::<Cache>::get_tasks_state_cache(env).await,
            LifecycleState::Off
        );

        start_env(env).await;

        assert_eq!(
            Tasks::<Cache>::get_tasks_state_cache(env).await,
            LifecycleState::Running
        );

        stop_env(env).await;
    }

    async fn scenario_cannot_remove_tasks_when_running(env: Environments) {
        start_env(env).await;

        let result = Tasks::<Cache>::remove_tasks_cache(env).await;
        assert!(result.is_err());

        stop_env(env).await;
    }

    // =========================
    // Entry point
    // =========================

    #[tokio::test]
    async fn cache_tasks_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_reset_env_clears_state(env).await;
        scenario_set_tasks_cache(env).await;
        scenario_upsert_task_insert(env).await;
        scenario_remove_task(env).await;
        scenario_get_tasks_state_cache(env).await;
        scenario_cannot_remove_tasks_when_running(env).await;
    }
}
