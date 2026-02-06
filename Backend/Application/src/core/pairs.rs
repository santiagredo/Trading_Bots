use crate::{
    handler::Pairs,
    logic,
    utils::{handle_user_err, AnyRepo, EntityCache, Repository, Response},
};
use models::{
    entities::pairs::Model,
    enums::LifecycleState,
    structs::{Environments, PairRequest, QueryOptions},
};

impl<R> Pairs<R>
where
    R: Repository<PairRequest, Model>,
{
    pub async fn insert(&self, req: PairRequest) -> Result<Model, Response> {
        logic::pairs::validate_insert(&req).map_err(handle_user_err)?;
        self.repo.insert(req).await
    }

    pub async fn select(&self, req: PairRequest) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }

    pub async fn select_many(
        &self,
        req: PairRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn update(&self, req: PairRequest) -> Result<Model, Response> {
        logic::pairs::validate_update(&req).map_err(handle_user_err)?;
        self.repo.update(req).await
    }

    pub async fn delete(&self, req: PairRequest) -> Result<u64, Response> {
        // logic::pairs::validate_delete(&req).map_err(handle_user_err)?;
        self.repo.delete(req).await
    }
}

impl Pairs<AnyRepo<PairRequest, Model>>
where
    Self: EntityCache<Environments>,
{
    pub async fn start_pairs(&self, env: Environments) -> Result<(), Response> {
        // STARTING
        self.set_state(env, LifecycleState::Starting)
            .await
            .map_err(Response::server_error)?;

        // Load pairs from DB
        let req = PairRequest::default();

        let values = match self.select_many(req, None).await {
            Ok(v) => v,
            Err(err) => {
                let _ = self.reset(env).await;
                return Err(err);
            }
        };

        // RUNNING
        self.set_state(env, LifecycleState::Running)
            .await
            .map_err(Response::server_error)?;

        if let Err(err) = self.set_all(env, values).await {
            let _ = self.reset(env).await;
            return Err(Response::server_error(err));
        }

        Ok(())
    }
}

impl<R> Pairs<R>
where
    Self: EntityCache<Environments>,
{
    pub async fn stop_pairs(&self, env: Environments) -> Result<(), Response> {
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
