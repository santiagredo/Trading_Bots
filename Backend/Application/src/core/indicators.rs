use crate::utils::EntityCache;
use crate::{
    handler::Indicators,
    utils::{Repository, Response},
};
use models::{
    entities::indicators::Model,
    enums::LifecycleState,
    structs::{Environments, IndicatorRequest, QueryOptions},
};

impl<R> Indicators<R>
where
    R: Repository<IndicatorRequest, Model>,
{
    pub async fn insert(&self, req: IndicatorRequest) -> Result<Model, Response> {
        self.repo.insert(req).await
    }

    pub async fn select(&self, req: IndicatorRequest) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }

    pub async fn select_many(
        &self,
        req: IndicatorRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn update(&self, req: IndicatorRequest) -> Result<Model, Response> {
        self.repo.update(req).await
    }

    pub async fn delete(&self, req: IndicatorRequest) -> Result<u64, Response> {
        self.repo.delete(req).await
    }
}

impl<R> Indicators<R>
where
    R: Repository<IndicatorRequest, Model> + Send + Sync + Clone + 'static,
{
    pub async fn start_indicators(&self, env: Environments) -> Result<(), Response> {
        /* ===========================
         * STARTING
         * ===========================
         */

        self.set_state(env, LifecycleState::Starting)
            .await
            .map_err(Response::server_error)?;

        /* ===========================
         * LOAD FROM DB
         * ===========================
         */

        let mut req = IndicatorRequest::default();
        req.is_active = Some(true);

        let indicators = match self.repo.select_many(req, None).await {
            Ok(i) => i,
            Err(err) => {
                let _ = self.reset(env).await;
                return Err(err);
            }
        };

        /* ===========================
         * RUNNING
         * ===========================
         */

        self.set_state(env, LifecycleState::Running)
            .await
            .map_err(Response::server_error)?;

        self.set_all(env, indicators).await.map_err(|e| {
            let _ = self.reset(env);
            Response::server_error(e)
        })?;

        Ok(())
    }
}

impl<R> Indicators<R>
where
    R: Send + Sync,
{
    pub async fn stop_indicators(&self, env: Environments) -> Result<(), Response> {
        self.set_state(env, LifecycleState::Stopping)
            .await
            .map_err(Response::server_error)?;

        self.remove_all(env).await.map_err(Response::server_error)?;

        self.set_state(env, LifecycleState::Off)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }

    pub async fn reset_indicators(&self, env: Environments) -> Result<(), Response> {
        self.reset(env).await.map_err(Response::server_error)
    }
}
