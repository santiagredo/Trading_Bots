use std::{collections::HashMap, future::Future, mem, sync::Arc, time::Duration};

use chrono::{Local, Timelike};
use migration::async_trait::async_trait;
use models::{
    entities::{critical_metrics, tasks::Model},
    enums::{transition_with_timestamp, FiniteStateMachine, LifecycleState},
    structs::{
        CacheTask, CacheTasks, CacheTasksEnvironments, Environments, MetricRequest, TaskRequest,
    },
};
use once_cell::sync::Lazy;
use tokio::{sync::RwLock, time::sleep};
use tokio_util::sync::CancellationToken;

use crate::{
    handler::{Binance, CoinPaprika, Metrics, Tasks},
    utils::{EntityCache, RepoFactory},
};

static ACTIVE_TASKS: Lazy<Arc<RwLock<CacheTasksEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheTasksEnvironments::new())));

#[async_trait]
impl<R> EntityCache<Environments> for Tasks<R>
where
    R: Send + Sync,
{
    type Key = i32;
    type Value = CacheTask;
    type Collection = CacheTasks;

    async fn state(&self, env: Environments) -> LifecycleState {
        let cache = ACTIVE_TASKS.read().await;
        cache
            .get(&env)
            .map(|c| c.status)
            .unwrap_or(LifecycleState::Off)
    }

    async fn set_state(&self, env: Environments, state: LifecycleState) -> Result<(), String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache.get_or_create(env);

        transition_with_timestamp(env_cache, state)?;
        Ok(())
    }

    async fn get_all(&self, env: Environments) -> Option<CacheTasks> {
        let cache = ACTIVE_TASKS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        Some(env_cache.clone())
    }

    async fn get(&self, env: Environments, key: i32) -> Option<CacheTask> {
        let cache = ACTIVE_TASKS.read().await;
        let env_cache = cache.get(&env)?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return None;
        }

        env_cache.models.get(&key).cloned()
    }

    async fn set_all(&self, env: Environments, values: Vec<CacheTask>) -> Result<(), String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models = values.into_iter().map(|v| (v.model.id, v)).collect();

        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    async fn upsert(&self, env: Environments, key: i32, value: CacheTask) -> Result<(), String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache.get_or_create(env);

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        env_cache.models.insert(key, value);
        env_cache.last_update_date = Local::now().naive_local();
        Ok(())
    }

    async fn remove(&self, env: Environments, key: i32) -> Result<Option<CacheTask>, String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Running) {
            return Err("Cache not running".into());
        }

        let removed = env_cache.models.remove(&key);
        env_cache.last_update_date = Local::now().naive_local();
        Ok(removed)
    }

    async fn remove_all(&self, env: Environments) -> Result<HashMap<i32, CacheTask>, String> {
        let mut cache = ACTIVE_TASKS.write().await;
        let env_cache = cache.get_mut(&env).ok_or("Environment not initialized")?;

        if !env_cache.status.allows(LifecycleState::Stopping) {
            return Err("Cache not stopping".into());
        }

        let removed = mem::take(&mut env_cache.models);
        env_cache.last_update_date = Local::now().naive_local();
        Ok(removed)
    }

    async fn reset(&self, env: Environments) -> Result<(), String> {
        let mut cache = ACTIVE_TASKS.write().await;

        if let Some(env_cache) = cache.environments.get_mut(&env) {
            *env_cache = CacheTasks::new();
        }

        Ok(())
    }
}

impl<R> Tasks<R>
where
    R: Clone + Send + Sync + 'static,
{
    fn spawn_task<F, Fut>(
        factory: RepoFactory,
        environment: Environments,
        task: Model,
        cancellation_token: CancellationToken,
        task_fn: F,
    ) where
        F: Fn(RepoFactory, Environments) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let delay = task.delay as u64;
        let cooldown = task.cooldown as u64;
        let task_id = task.id;

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => break,
                    _ = sleep(Duration::from_secs(delay)) => {
                        task_fn(factory.clone(), environment).await;

                        let now = Local::now().naive_local();

                        let repo = factory.repo::<TaskRequest, Model>();
                        let service = Tasks::new(repo);

                        let _ = service.update(
                            TaskRequest {
                                id: Some(task_id),
                                last_execution: Some(now),
                                ..Default::default()
                            }
                        ).await;

                        sleep(Duration::from_secs(cooldown)).await;
                    }
                }
            }
        });
    }

    pub fn run_task(
        factory: RepoFactory,
        environment: Environments,
        task: Model,
        cancellation_token: CancellationToken,
    ) {
        match task.nick.as_str() {
            "BNUAB" => Self::spawn_task(
                factory,
                environment,
                task,
                cancellation_token,
                |factory, env| async move {
                    Binance::update_account_balances(factory, env).await;
                },
            ),
            "BNUEI" => Self::spawn_task(
                factory,
                environment,
                task,
                cancellation_token,
                |factory, env| async move {
                    Binance::update_exchange_information(factory, env).await;
                },
            ),
            "CPUPS" => Self::spawn_task(
                factory,
                environment,
                task,
                cancellation_token,
                |factory, env| async move {
                    CoinPaprika::new()
                        .update_pairs_statistics(factory, env)
                        .await;
                },
            ),
            "CMPER" => Self::spawn_task(
                factory,
                environment,
                task,
                cancellation_token,
                |factory, env| async move {
                    let now = Local::now();
                    let next_hour = (now + chrono::Duration::hours(1))
                        .with_minute(0)
                        .and_then(|t| t.with_second(0))
                        .and_then(|t| t.with_nanosecond(0))
                        .unwrap();

                    sleep((next_hour - now).to_std().unwrap()).await;

                    let repo = factory.repo::<MetricRequest, critical_metrics::Model>();
                    let _ = Metrics::new(repo).persist(env).await;
                },
            ),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use models::{
        entities::tasks::Model,
        enums::{LifecycleState, TaskState},
        structs::{CacheTask, Environments},
    };

    use crate::{handler::Tasks, utils::EntityCache};

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

    async fn new_service() -> Tasks<()> {
        Tasks::blank()
    }

    async fn start_env(service: &Tasks<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Starting)
            .await
            .unwrap();

        service
            .set_state(env, LifecycleState::Running)
            .await
            .unwrap();
    }

    async fn stop_env(service: &Tasks<()>, env: Environments) {
        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert!(removed.is_empty() || !removed.is_empty());

        service.set_state(env, LifecycleState::Off).await.unwrap();
    }

    // =========================
    // Scenarios
    // =========================

    async fn scenario_reset_env_clears_state(service: &Tasks<()>, env: Environments) {
        start_env(service, env).await;

        let task = CacheTask {
            model: mock_task(1),
            ..Default::default()
        };

        service.set_all(env, vec![task]).await.unwrap();

        service
            .set_state(env, LifecycleState::Stopping)
            .await
            .unwrap();

        let removed = service.remove_all(env).await.unwrap();
        assert_eq!(removed.len(), 1);

        service.set_state(env, LifecycleState::Off).await.unwrap();

        assert_eq!(service.state(env).await, LifecycleState::Off);
    }

    async fn scenario_set_and_get_all(service: &Tasks<()>, env: Environments) {
        start_env(service, env).await;

        let tasks = vec![
            CacheTask {
                model: mock_task(1),
                ..Default::default()
            },
            CacheTask {
                model: mock_task(2),
                ..Default::default()
            },
        ];

        service.set_all(env, tasks).await.unwrap();

        let cache = service.get_all(env).await.unwrap();
        assert_eq!(cache.models.len(), 2);
        assert!(cache.models.contains_key(&1));
        assert!(cache.models.contains_key(&2));

        stop_env(service, env).await;
    }

    async fn scenario_upsert_task(service: &Tasks<()>, env: Environments) {
        start_env(service, env).await;

        let task = CacheTask {
            model: mock_task(10),
            ..Default::default()
        };

        service.upsert(env, 10, task).await.unwrap();

        let cached = service.get(env, 10).await.unwrap();
        assert_eq!(cached.model.id, 10);
        assert_eq!(cached.state, TaskState::Sleeping);

        stop_env(service, env).await;
    }

    async fn scenario_remove_task(service: &Tasks<()>, env: Environments) {
        start_env(service, env).await;

        let task = CacheTask {
            model: mock_task(20),
            ..Default::default()
        };

        service.upsert(env, 20, task).await.unwrap();

        let removed = service.remove(env, 20).await.unwrap();
        assert!(removed.is_some());

        let cached = service.get(env, 20).await;
        assert!(cached.is_none());

        stop_env(service, env).await;
    }

    async fn scenario_get_state(service: &Tasks<()>, env: Environments) {
        assert_eq!(service.state(env).await, LifecycleState::Off);

        start_env(service, env).await;
        assert_eq!(service.state(env).await, LifecycleState::Running);

        stop_env(service, env).await;
    }

    async fn scenario_cannot_remove_all_when_running(service: &Tasks<()>, env: Environments) {
        start_env(service, env).await;

        let result = service.remove_all(env).await;
        assert!(result.is_err());

        stop_env(service, env).await;
    }

    // =========================
    // Entry point
    // =========================

    #[tokio::test]
    async fn cache_tasks_unit_responsibilities() {
        let env = Environments::DEV;
        let service = new_service().await;

        scenario_reset_env_clears_state(&service, env).await;
        scenario_set_and_get_all(&service, env).await;
        scenario_upsert_task(&service, env).await;
        scenario_remove_task(&service, env).await;
        scenario_get_state(&service, env).await;
        scenario_cannot_remove_all_when_running(&service, env).await;
    }
}
