use models::{
    entities::tasks::Model,
    enums::LifecycleState,
    structs::{CacheTask, CacheTasks, QueryOptions},
};
use tokio_util::sync::CancellationToken;

use crate::{
    handler::{Tasks, DBC},
    utils::{handle_user_err, Cache, Core, Response},
};

impl Tasks<Core> {
    /* ===========================
     * DB
     * ===========================
     */

    pub async fn select_tasks_core(
        self,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase()
            .select_tasks_data(&DBC::db(&env).await?, query)
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

    pub async fn get_task_core(self, task_id: i32) -> Option<CacheTask> {
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

    pub async fn remove_task_core(self, task_id: i32) -> Result<Option<CacheTask>, Response> {
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

        let mut tasks_request = Tasks::default().with_env(env);
        tasks_request.model.is_active = Some(true);

        let tasks = match tasks_request.select_tasks(None).await {
            Ok(t) => t,
            Err(err) => {
                let _ = Tasks::<Cache>::reset_tasks_cache(env).await;
                return Err(err);
            }
        };

        for task in tasks.iter() {
            Tasks::<Cache>::run_task(task.clone(), env, cancellation_token.clone());
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
}
