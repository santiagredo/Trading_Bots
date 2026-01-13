use std::{future::Future, time::Duration};

use chrono::{Local, Timelike};
use models::{
    entities::tasks::Model,
    enums::LifecycleState,
    structs::{CacheTasks, Environments},
};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

use crate::{
    handler::{Binance, CoinPaprika, Metrics, Tasks, DBC},
    utils::{handle_user_err, Cache, Core, Response},
};

impl Tasks<Core> {
    /* ===========================
     * DB
     * ===========================
     */

    pub async fn select_tasks_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase()
            .select_tasks_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn update_task_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase()
            .update_task_logic()
            .map_err(handle_user_err)?
            .next_phase()
            .update_task_data(&DBC::db(&env).await?)
            .await
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_tasks_core(self) -> Option<CacheTasks> {
        Tasks::<Cache>::get_tasks_cache(self.environment).await
    }

    pub async fn get_task_core(self, task_id: i32) -> Option<Model> {
        Tasks::<Cache>::get_task_cache(self.environment, task_id).await
    }

    pub async fn get_tasks_state_core(self) -> LifecycleState {
        Tasks::<Cache>::get_tasks_state_cache(self.environment).await
    }

    /* ===========================
     * CACHE (WRITE)
     * ===========================
     */

    pub async fn upsert_task_core(self, task: Model) -> Result<(), Response> {
        Tasks::<Cache>::upsert_task_cache(self.environment, task)
            .await
            .map_err(|e| Response::server_error(e))
    }

    pub async fn remove_task_core(self, task_id: i32) -> Result<Option<Model>, Response> {
        Tasks::<Cache>::remove_task_cache(self.environment, task_id)
            .await
            .map_err(|e| Response::server_error(e))
    }

    pub async fn reset_tasks_core(self) -> Result<(), Response> {
        Tasks::<Cache>::reset_tasks_cache(self.environment)
            .await
            .map_err(|e| Response::server_error(e))
    }

    /* ===========================
     * START ACTIVE TASKS
     * ===========================
     */

    pub async fn start_tasks_core(
        self,
        cancellation_token: &CancellationToken,
    ) -> Result<(), Response> {
        let env = self.environment;

        if Tasks::<Cache>::get_tasks_state_cache(self.environment).await == LifecycleState::Running
        {
            return Ok(());
        }

        if let Err(err) =
            Tasks::<Cache>::set_status_cache(env, models::enums::LifecycleState::Starting).await
        {
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        let tasks = match Tasks::default().with_env(env).select_tasks().await {
            Ok(t) => t,
            Err(err) => {
                let _ = Tasks::<Cache>::reset_tasks_cache(env).await;
                return Err(err);
            }
        };

        for task in tasks.iter() {
            Self::run_tasks(task.clone(), env, cancellation_token.clone());
        }

        if let Err(err) =
            Tasks::<Cache>::set_status_cache(env, models::enums::LifecycleState::Running).await
        {
            let _ = Tasks::<Cache>::reset_tasks_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        if let Err(err) = Tasks::<Cache>::set_tasks_cache(env, tasks).await {
            let _ = Tasks::<Cache>::reset_tasks_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    /* ===========================
     * STOP ACTIVE TASKS
     * ===========================
     */

    pub async fn stop_tasks_core(self) -> Result<(), Response> {
        let env = self.environment;

        if let Err(err) =
            Tasks::<Cache>::set_status_cache(env, models::enums::LifecycleState::Stopping).await
        {
            let _ = Tasks::<Cache>::reset_tasks_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        if let Err(err) = Tasks::<Cache>::remove_tasks_cache(env).await {
            let _ = Tasks::<Cache>::reset_tasks_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        if let Err(err) =
            Tasks::<Cache>::set_status_cache(env, models::enums::LifecycleState::Off).await
        {
            let _ = Tasks::<Cache>::reset_tasks_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    fn run_tasks(task: Model, environment: Environments, cancellation_token: CancellationToken) {
        let (delay, cooldown) = (task.delay as u64, task.cooldown as u64);

        match task.nick.as_str() {
            "BNUAB" => Self::spawn_task(
                delay,
                cooldown,
                environment,
                cancellation_token,
                |env| async move {
                    Binance::update_account_balances(env).await;
                },
            ),
            "BNUEI" => Self::spawn_task(
                delay,
                cooldown,
                environment,
                cancellation_token,
                |env| async move {
                    Binance::update_exchange_information(env).await;
                },
            ),
            "CPUPS" => Self::spawn_task(
                delay,
                cooldown,
                environment,
                cancellation_token,
                |env| async move {
                    CoinPaprika::default()
                        .with_env(env)
                        .update_pairs_statistics()
                        .await;
                },
            ),
            "CMPER" => Self::spawn_task(
                delay,
                cooldown,
                environment,
                cancellation_token,
                |env| async move {
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
                            return;
                        }
                    };

                    let wait = match (next_hour - now).to_std() {
                        Ok(d) => d,
                        Err(_) => Duration::from_secs(3600),
                    };

                    sleep(wait).await;

                    let _ = Metrics::default().with_env(env).persist_metrics().await;
                },
            ),
            _ => {}
        }
    }

    fn spawn_task<F, Fut>(
        delay: u64,
        cooldown: u64,
        environment: Environments,
        cancellation_token: CancellationToken,
        task_fn: F,
    ) where
        F: Fn(Environments) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        break;
                    }
                    _ = sleep(Duration::from_secs(delay)) => {
                        task_fn(environment).await;

                        tokio::select! {
                            _ = cancellation_token.cancelled() => {
                                break;
                            }
                            _ = sleep(Duration::from_secs(cooldown)) => {}
                        }
                    }
                }
            }
        });
    }
}
