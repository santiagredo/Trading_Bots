use crate::{
    handler::Tasks,
    logic::tasks::update_task_logic,
    utils::{handle_user_err, EntityCache, RepoFactory, Repository, Response},
};
use models::{
    entities::tasks::Model,
    enums::{LifecycleState, TaskState},
    structs::{CacheTask, Environments, QueryOptions, TaskRequest},
};

/* ======================================================
 * DB / CORE
 * ======================================================
 */

impl<R> Tasks<R>
where
    R: Repository<TaskRequest, Model>,
{
    pub async fn select(
        &self,
        req: TaskRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn update(&self, mut req: TaskRequest) -> Result<Model, Response> {
        update_task_logic(&mut req).map_err(handle_user_err)?;
        self.repo.update(req).await
    }
}

/* ======================================================
 * START
 * ======================================================
 */

impl Tasks<crate::utils::AnyRepo<TaskRequest, Model>>
where
    Self: EntityCache<Environments>,
{
    pub async fn start(&self, factory: RepoFactory, env: Environments) -> Result<(), Response> {
        if self.state(env).await == LifecycleState::Running {
            return Ok(());
        }

        // STARTING
        self.set_state(env, LifecycleState::Starting)
            .await
            .map_err(Response::server_error)?;

        let mut req = TaskRequest::default();
        req.is_active = Some(true);

        let tasks = match self.select(req, None).await {
            Ok(v) => v,
            Err(err) => {
                let _ = self.reset(env).await;
                return Err(err);
            }
        };

        let mut cache_tasks = Vec::new();

        for task in tasks.into_iter() {
            let abort_handle = Self::run_task(factory.clone(), env, task.clone());

            let cache_task = CacheTask {
                model: task,
                state: TaskState::Sleeping,
                abort_handle,
            };

            cache_tasks.push(cache_task);
        }

        // RUNNING
        self.set_state(env, LifecycleState::Running)
            .await
            .map_err(Response::server_error)?;

        if let Err(err) = self.set_all(env, cache_tasks).await {
            let _ = self.reset(env).await;
            return Err(Response::server_error(err));
        }

        Ok(())
    }
}

/* ======================================================
 * STOP
 * ======================================================
 */

impl<R> Tasks<R>
where
    R: Send + Sync,
{
    pub async fn stop(&self, env: Environments) -> Result<(), Response> {
        self.set_state(env, LifecycleState::Stopping)
            .await
            .map_err(Response::server_error)?;

        let cache_tasks: Vec<CacheTask> = self
            .remove_all(env)
            .await
            .map_err(Response::server_error)?
            .into_values()
            .collect();

        for task in cache_tasks {
            if let Some(abort_handle) = task.abort_handle {
                abort_handle.abort();
            }
        }

        self.set_state(env, LifecycleState::Off)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }
}
