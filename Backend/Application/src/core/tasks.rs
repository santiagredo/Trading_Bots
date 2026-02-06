use crate::{
    handler::{Tasks, DBC},
    logic::tasks::update_task_logic,
    utils::{handle_user_err, RepoFactory, Response},
};
use models::{
    entities::tasks::Model,
    enums::LifecycleState,
    structs::{CacheTask, CacheTasks, Environments, QueryOptions, TaskRequest},
};
use tokio_util::sync::CancellationToken;

impl Tasks {
    /* ===========================
     * DB
     * ===========================
     */

    pub async fn select_tasks_core(
        self,
        env: Environments,
        task: TaskRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.select_tasks_data(&DBC::db(&env).await?, task, query)
            .await
    }

    pub async fn update_task_core(
        self,
        env: Environments,
        mut task: TaskRequest,
    ) -> Result<Model, Response> {
        update_task_logic(&mut task).map_err(handle_user_err)?;
        self.update_task_data(&DBC::db(&env).await?, task).await
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_tasks_core(self, env: Environments) -> Option<CacheTasks> {
        Tasks::get_tasks_cache(env).await
    }

    pub async fn get_task_core(self, env: Environments, task_id: i32) -> Option<CacheTask> {
        Tasks::get_task_cache(env, task_id).await
    }

    pub async fn get_tasks_state_core(self, env: Environments) -> LifecycleState {
        Tasks::get_tasks_state_cache(env).await
    }

    /* ===========================
     * CACHE (WRITE)
     * ===========================
     */

    pub async fn upsert_task_core(self, env: Environments, task: Model) -> Result<(), Response> {
        Tasks::upsert_task_cache(env, task)
            .await
            .map_err(|e| Response::server_error(e))
    }

    pub async fn remove_task_core(
        self,
        env: Environments,
        task_id: i32,
    ) -> Result<Option<CacheTask>, Response> {
        Tasks::remove_task_cache(env, task_id)
            .await
            .map_err(|e| Response::server_error(e))
    }

    pub async fn reset_tasks_core(self, env: Environments) -> Result<(), Response> {
        Tasks::reset_tasks_cache(env)
            .await
            .map_err(|e| Response::server_error(e))
    }

    /* ===========================
     * START ACTIVE TASKS
     * ===========================
     */

    pub async fn start_tasks_core(
        self,
        factory: RepoFactory,
        env: Environments,
        cancellation_token: &CancellationToken,
    ) -> Result<(), Response> {
        if Tasks::get_tasks_state_cache(env).await == LifecycleState::Running {
            return Ok(());
        }

        if let Err(err) =
            Tasks::set_status_cache(env, models::enums::LifecycleState::Starting).await
        {
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        let tasks_request = TaskRequest {
            is_active: Some(true),
            ..Default::default()
        };

        let tasks = match Tasks.select_tasks(env, tasks_request, None).await {
            Ok(t) => t,
            Err(err) => {
                let _ = Tasks::reset_tasks_cache(env).await;
                return Err(err);
            }
        };

        for task in tasks.iter() {
            Tasks::run_task(
                factory.clone(),
                env,
                task.clone(),
                cancellation_token.clone(),
            );
        }

        if let Err(err) = Tasks::set_status_cache(env, models::enums::LifecycleState::Running).await
        {
            let _ = Tasks::reset_tasks_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        if let Err(err) = Tasks::set_tasks_cache(env, tasks).await {
            let _ = Tasks::reset_tasks_cache(env).await;
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

    pub async fn stop_tasks_core(self, env: Environments) -> Result<(), Response> {
        if let Err(err) =
            Tasks::set_status_cache(env, models::enums::LifecycleState::Stopping).await
        {
            let _ = Tasks::reset_tasks_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        if let Err(err) = Tasks::remove_tasks_cache(env).await {
            let _ = Tasks::reset_tasks_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        if let Err(err) = Tasks::set_status_cache(env, models::enums::LifecycleState::Off).await {
            let _ = Tasks::reset_tasks_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }
}
