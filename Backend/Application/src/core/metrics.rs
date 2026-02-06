use crate::utils::{AnyRepo, EntityCache, Repository, Response};
use models::{
    entities::critical_metrics::Model,
    enums::LifecycleState,
    structs::{Environments, MetricRequest, QueryOptions},
};

use crate::handler::Metrics;

/* ======================================================
 * CRUD / DB
 * ======================================================
 */

impl<R> Metrics<R>
where
    R: Repository<MetricRequest, Model>,
{
    pub async fn insert(&self, req: MetricRequest) -> Result<Model, Response> {
        self.repo.insert(req).await
    }

    pub async fn select(&self, req: MetricRequest) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }

    pub async fn select_many(
        &self,
        req: MetricRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn update(&self, req: MetricRequest) -> Result<Model, Response> {
        self.repo.update(req).await
    }

    pub async fn delete(&self, req: MetricRequest) -> Result<u64, Response> {
        self.repo.delete(req).await
    }
}

/* ======================================================
 * START / LOAD CACHE
 * ======================================================
 */

impl Metrics<AnyRepo<MetricRequest, Model>>
where
    Self: EntityCache<Environments>,
{
    pub async fn start(
        &self,
        // factory: RepoFactory,
        env: Environments,
    ) -> Result<(), Response> {
        // STARTING
        self.set_state(env, LifecycleState::Starting)
            .await
            .map_err(Response::server_error)?;

        // RUNNING
        self.set_state(env, LifecycleState::Running)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }
}

impl<R> Metrics<R>
where
    R: Repository<MetricRequest, Model> + Send + Sync,
{
    pub async fn persist(&self, env: Environments) -> Result<Model, Response> {
        let metric = match self.remove(env, ()).await {
            Ok(Some(val)) => Metrics::into_request(val),
            Ok(None) => {
                return Err(Response {
                    code: 500,
                    message: format!("Metric not found"),
                })
            }
            Err(err) => {
                return Err(Response {
                    code: 500,
                    message: err,
                })
            }
        };

        self.repo.insert(metric).await
    }
}

/* ======================================================
 * STOP / RESET
 * ======================================================
 */

impl<R> Metrics<R>
where
    Self: EntityCache<Environments>,
{
    pub async fn stop_metrics(&self, env: Environments) -> Result<(), Response> {
        self.set_state(env, LifecycleState::Stopping)
            .await
            .map_err(Response::server_error)?;

        self.remove_all(env).await.map_err(Response::server_error)?;

        self.set_state(env, LifecycleState::Off)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }
}
