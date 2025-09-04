use std::{collections::HashMap, sync::Arc, time::Duration};

use models::entities::tasks::Model;
use once_cell::sync::Lazy;
use tokio::{sync::RwLock, task::AbortHandle, time::sleep};

use crate::{
    handler::{Binance, CoinPaprika, Tasks},
    utils::{Cache, Response},
};

static ACTIVE_TASKS: Lazy<Arc<RwLock<Option<HashMap<i32, (Model, AbortHandle)>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Tasks<Cache> {
    pub async fn set_active_tasks_cache(tasks: Option<Vec<Model>>) -> Option<Vec<Model>> {
        let mut active_tasks = ACTIVE_TASKS.write().await;

        let Some(tasks) = tasks else {
            *active_tasks = None;
            return None;
        };

        let mut active_tasks_map: HashMap<i32, (Model, AbortHandle)> = HashMap::new();

        for task in tasks.iter() {
            if let Some(abort_handle) = Tasks::get_abort_handle(task).await {
                active_tasks_map.insert(task.id, (task.clone(), abort_handle));
            }
        }

        *active_tasks = Some(active_tasks_map);

        Some(tasks)
    }

    pub async fn set_active_task_cache(task: Model) -> Model {
        let mut active_tasks = ACTIVE_TASKS.write().await;

        let Some(tasks_map) = active_tasks.as_mut() else {
            return task;
        };

        if let Some((_, abort_handle)) = tasks_map.remove(&task.id) {
            abort_handle.abort();
        };

        if !task.is_active {
            return task;
        }

        if let Some(abort_handle) = Tasks::<Cache>::get_abort_handle(&task).await {
            tasks_map.insert(task.id, (task.clone(), abort_handle));
        };

        task
    }

    pub async fn get_active_tasks_cache() -> Option<Vec<Model>> {
        let active_tasks = ACTIVE_TASKS.read().await;

        if let Some(tasks) = active_tasks.as_ref() {
            let models = tasks
                .iter()
                .map(|(_, (model, _))| model.clone())
                .collect::<Vec<Model>>();

            return Some(models);
        }

        None
    }

    pub async fn get_active_task_cache(key: &i32) -> Option<Model> {
        let active_tasks = ACTIVE_TASKS.read().await;

        let Some(tasks_map) = active_tasks.as_ref() else {
            return None;
        };

        let Some((model, _)) = tasks_map.get(key).cloned() else {
            return None;
        };

        Some(model)
    }

    pub async fn start_async_tasks_cache() -> Result<(), Response> {
        if Tasks::get_active_tasks_cache()
            .await
            .is_none_or(|map| map.is_empty())
        {
            let mut tasks_request = Tasks::default();
            tasks_request.model.is_active = Some(true);
            let active_tasks = tasks_request.select_tasks().await?;
            Tasks::set_active_tasks_cache(Some(active_tasks)).await;
        }

        Ok(())
    }

    pub async fn stop_async_tasks_cache() {
        let mut tasks_abort_handles = ACTIVE_TASKS.write().await;

        if let Some(map) = tasks_abort_handles.as_mut() {
            for (_, abort_handle) in map.values() {
                abort_handle.abort();
            }
        }

        *tasks_abort_handles = None;
    }

    pub async fn get_abort_handle(task: &Model) -> Option<AbortHandle> {
        let (delay, cooldown) = (task.delay as u64, task.cooldown as u64);

        match task.nick.as_ref() {
            "BNUAB" => Some(
                tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(delay)).await;
                        Binance::update_account_balances().await;
                        sleep(Duration::from_secs(cooldown)).await;
                    }
                })
                .abort_handle(),
            ),
            "BNUEI" => Some(
                tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(delay)).await;
                        Binance::update_exchange_information().await;
                        sleep(Duration::from_secs(cooldown)).await;
                    }
                })
                .abort_handle(),
            ),
            "CPUPS" => Some(
                tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(delay)).await;
                        CoinPaprika::update_pairs_statistics().await;
                        sleep(Duration::from_secs(cooldown)).await;
                    }
                })
                .abort_handle(),
            ),

            _ => None,
        }
    }
}
