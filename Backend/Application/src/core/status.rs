use crate::{
    handler::OrderStatus,
    utils::{AnyRepo, EntityCache, RepoFactory, Repository, Response},
};
use models::{entities::status::Model, enums::LifecycleState, structs::Environments};

/* ======================================================
 * CRUD / DB
 * ======================================================
 */

impl<R> OrderStatus<R>
where
    R: Repository<Model, Model>,
{
    pub async fn select(&self, req: Model) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }
    pub async fn select_many(&self, req: Model) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, None).await
    }
}

/* ======================================================
 * START / LOAD CACHE
 * ======================================================
 */

impl OrderStatus<AnyRepo<Model, Model>>
where
    Self: EntityCache<Environments>,
{
    pub async fn start(&self, factory: RepoFactory, env: Environments) -> Result<(), Response> {
        // =========================
        // STARTING
        // =========================
        self.set_state(env, LifecycleState::Starting)
            .await
            .map_err(Response::server_error)?;

        // =========================
        // Load statuses from DB
        // =========================
        let repo = factory.repo();
        let service = OrderStatus::new(repo);

        let statuses = match service.select_many(Model::default()).await {
            Ok(v) => v,
            Err(err) => {
                let _ = self.reset(env).await;
                return Err(err);
            }
        };

        // =========================
        // RUNNING
        // =========================
        self.set_state(env, LifecycleState::Running)
            .await
            .map_err(Response::server_error)?;

        // =========================
        // Populate cache
        // =========================
        if let Err(err) = self.set_all(env, statuses).await {
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

impl<R> OrderStatus<R>
where
    Self: EntityCache<Environments>,
{
    pub async fn stop(&self, env: Environments) -> Result<(), Response> {
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
