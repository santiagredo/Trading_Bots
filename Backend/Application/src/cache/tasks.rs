use std::{collections::HashMap, sync::Arc, time::Duration};

use chrono::{Local, Timelike};
use models::{entities::tasks::Model, structs::Environments};
use once_cell::sync::Lazy;
use tokio::{sync::RwLock, task::AbortHandle, time::sleep};

use crate::{
    handler::{Binance, CoinPaprika, Metrics, Tasks},
    utils::Cache,
};

#[derive(Default)]
struct CacheEnvironments {
    pub environments: HashMap<Environments, CacheTasks>,
}

#[derive(Default)]
struct CacheTasks {
    pub is_initialized: bool,
    pub models: HashMap<i32, (Model, AbortHandle)>,
}

static ACTIVE_TASKS: Lazy<Arc<RwLock<CacheEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheEnvironments::default())));

impl Tasks<Cache> {
    pub async fn set_active_tasks_cache(
        environment: &Environments,
        tasks: Vec<Model>,
    ) -> Vec<Model> {
        let mut cache_tasks = ACTIVE_TASKS.write().await;

        let env_map = cache_tasks
            .environments
            .entry(*environment)
            .or_insert_with(CacheTasks::default);

        for (_, (_, abort_handle)) in env_map.models.drain() {
            abort_handle.abort();
        }

        for task in tasks.iter() {
            if let Some(abort_handle) = Tasks::<Cache>::get_abort_handle(task, *environment).await {
                env_map.models.insert(task.id, (task.clone(), abort_handle));
            }
        }

        env_map.is_initialized = true;

        tasks
    }

    pub async fn set_active_task_cache(
        environment: &Environments,
        task: Model,
        is_remove: bool,
    ) -> Model {
        let mut cache_tasks = ACTIVE_TASKS.write().await;

        let Some(env_map) = cache_tasks.environments.get_mut(environment) else {
            return task;
        };

        if let Some((_, abort_handle)) = env_map.models.remove(&task.id) {
            abort_handle.abort();
        }

        if is_remove || !task.is_active {
            return task;
        }

        if let Some(abort_handle) = Self::get_abort_handle(&task, *environment).await {
            env_map.models.insert(task.id, (task.clone(), abort_handle));
        }

        task
    }

    pub async fn get_active_tasks_cache(environment: &Environments) -> Option<Vec<Model>> {
        let cache_tasks = ACTIVE_TASKS.read().await;

        let env_map = cache_tasks.environments.get(environment)?;

        let tasks = env_map
            .models
            .values()
            .map(|(model, _)| model.clone())
            .collect::<Vec<_>>();

        Some(tasks)
    }

    pub async fn get_active_task_cache(environment: &Environments, key: &i32) -> Option<Model> {
        let cache_tasks = ACTIVE_TASKS.read().await;

        let env_map = cache_tasks.environments.get(environment)?;

        let (model, _) = env_map.models.get(key)?.clone();

        Some(model)
    }

    pub async fn stop_active_tasks_cache(environment: &Environments) {
        let mut cache_tasks = ACTIVE_TASKS.write().await;

        let Some(env_map) = cache_tasks.environments.get_mut(environment) else {
            return;
        };

        for (_, (_, abort_handle)) in env_map.models.drain() {
            abort_handle.abort();
        }

        env_map.is_initialized = false;
    }

    pub async fn get_active_tasks_status_cache(environment: &Environments) -> bool {
        let cache_tasks = ACTIVE_TASKS.read().await;

        cache_tasks
            .environments
            .get(&environment)
            .map(|val| val.is_initialized)
            .unwrap_or(false)
    }

    async fn get_abort_handle(task: &Model, environment: Environments) -> Option<AbortHandle> {
        let (delay, cooldown) = (task.delay as u64, task.cooldown as u64);

        match task.nick.as_ref() {
            "BNUAB" => Some(
                tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(delay)).await;
                        Binance::update_account_balances(environment).await;
                        sleep(Duration::from_secs(cooldown)).await;
                    }
                })
                .abort_handle(),
            ),
            "BNUEI" => Some(
                tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(delay)).await;
                        Binance::update_exchange_information(environment).await;
                        sleep(Duration::from_secs(cooldown)).await;
                    }
                })
                .abort_handle(),
            ),
            "CPUPS" => Some(
                tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(delay)).await;
                        CoinPaprika::default()
                            .with_env(environment)
                            .update_pairs_statistics()
                            .await;
                        sleep(Duration::from_secs(cooldown)).await;
                    }
                })
                .abort_handle(),
            ),
            "CMPER" => Some(
                tokio::spawn(async move {
                    loop {
                        let now = Local::now();

                        let next_hour = match (now + chrono::Duration::hours(1))
                            .with_minute(0)
                            .and_then(|t| t.with_second(0))
                            .and_then(|t| t.with_nanosecond(0))
                        {
                            Some(t) => t,
                            None => {
                                tracing::error!("Failed to compute next hour, retrying in 60s");
                                sleep(Duration::from_secs(60)).await;
                                continue;
                            }
                        };

                        let wait = match (next_hour - now).to_std() {
                            Ok(d) => d,
                            Err(_) => Duration::from_secs(3600),
                        };

                        sleep(wait).await;

                        let _ = Metrics::default()
                            .with_env(environment)
                            .persist_metrics()
                            .await;
                    }
                })
                .abort_handle(),
            ),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use models::{entities::tasks::Model, structs::Environments};

    use crate::{handler::Tasks, utils::Cache};

    // Helpers

    fn mock_task(id: i32, nick: &str, is_active: bool) -> Model {
        Model {
            id,
            nick: nick.to_string(),
            delay: 10000,
            cooldown: 10000,
            is_active,
            ..Default::default()
        }
    }

    async fn reset_env(env: Environments) {
        Tasks::<Cache>::stop_active_tasks_cache(&env).await;
    }

    // Scenarios

    // Initial state
    async fn scenario_initial_state(env: Environments) {
        reset_env(env).await;

        let tasks = Tasks::<Cache>::get_active_tasks_cache(&env).await;
        let status = Tasks::<Cache>::get_active_tasks_status_cache(&env).await;

        assert!(tasks.is_none());
        assert!(!status);
    }

    // Bulk set
    async fn scenario_set_active_tasks_cache(env: Environments) {
        reset_env(env).await;

        let tasks = vec![mock_task(1, "BNUAB", true), mock_task(2, "BNUEI", true)];

        Tasks::<Cache>::set_active_tasks_cache(&env, tasks.clone()).await;

        let cached = Tasks::<Cache>::get_active_tasks_cache(&env)
            .await
            .expect("tasks should exist");

        let status = Tasks::<Cache>::get_active_tasks_status_cache(&env).await;

        assert_eq!(cached.len(), 2);
        assert!(status);

        reset_env(env).await;
    }

    // Single insert
    async fn scenario_set_active_task_cache_insert(env: Environments) {
        reset_env(env).await;

        let task = mock_task(10, "CPUPS", true);

        Tasks::<Cache>::set_active_task_cache(&env, task.clone(), false).await;

        let cached = Tasks::<Cache>::get_active_task_cache(&env, &10).await;

        assert_eq!(cached, Some(task));

        reset_env(env).await;
    }

    // Single remove
    async fn scenario_set_active_task_cache_remove(env: Environments) {
        reset_env(env).await;

        let task = mock_task(20, "BNUAB", true);

        Tasks::<Cache>::set_active_task_cache(&env, task.clone(), false).await;
        Tasks::<Cache>::set_active_task_cache(&env, task.clone(), true).await;

        let cached = Tasks::<Cache>::get_active_task_cache(&env, &20).await;

        assert!(cached.is_none());

        reset_env(env).await;
    }

    // Inactive task should not be inserted
    async fn scenario_inactive_task_is_not_inserted(env: Environments) {
        reset_env(env).await;

        let task = mock_task(30, "BNUEI", false);

        Tasks::<Cache>::set_active_task_cache(&env, task, false).await;

        let cached = Tasks::<Cache>::get_active_tasks_cache(&env).await;

        assert!(cached.is_some_and(|val| val.is_empty()));

        reset_env(env).await;
    }

    // Get all tasks
    async fn scenario_get_active_tasks_cache(env: Environments) {
        reset_env(env).await;

        let tasks = vec![
            mock_task(1, "BNUAB", true),
            mock_task(2, "BNUEI", true),
            mock_task(3, "CPUPS", true),
        ];

        Tasks::<Cache>::set_active_tasks_cache(&env, tasks.clone()).await;

        let cached = Tasks::<Cache>::get_active_tasks_cache(&env).await.unwrap();

        assert_eq!(cached.len(), 3);

        reset_env(env).await;
    }

    // Stop clears tasks
    async fn scenario_stop_active_tasks_cache(env: Environments) {
        reset_env(env).await;

        let tasks = vec![mock_task(1, "BNUAB", true)];

        Tasks::<Cache>::set_active_tasks_cache(&env, tasks).await;
        Tasks::<Cache>::stop_active_tasks_cache(&env).await;

        let cached = Tasks::<Cache>::get_active_tasks_cache(&env).await;
        let status = Tasks::<Cache>::get_active_tasks_status_cache(&env).await;

        assert!(cached.unwrap_or_default().is_empty());
        assert!(!status);
    }

    #[tokio::test]
    async fn cache_tasks_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_initial_state(env).await;
        scenario_set_active_tasks_cache(env).await;
        scenario_set_active_task_cache_insert(env).await;
        scenario_set_active_task_cache_remove(env).await;
        scenario_inactive_task_is_not_inserted(env).await;
        scenario_get_active_tasks_cache(env).await;
        scenario_stop_active_tasks_cache(env).await;

        reset_env(env).await;
    }
}
