use crate::{
    handler::Integrations,
    logic,
    utils::{handle_user_err, EntityCache, Repository, Response},
};
use models::{
    entities::integrations::Model,
    enums::LifecycleState,
    structs::{Environments, IntegrationRequest, QueryOptions},
};

impl<R> Integrations<R>
where
    R: Repository<IntegrationRequest, Model>,
{
    pub async fn insert(&self, req: IntegrationRequest) -> Result<Model, Response> {
        logic::integrations::validate_insert(&req).map_err(handle_user_err)?;

        self.repo.insert(req).await
    }

    pub async fn select(&self, req: IntegrationRequest) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }

    pub async fn select_many(
        &self,
        req: IntegrationRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn update(&self, req: IntegrationRequest) -> Result<Model, Response> {
        logic::integrations::validate_update(&req).map_err(handle_user_err)?;

        self.repo.update(req).await
    }

    pub async fn delete(&self, req: IntegrationRequest) -> Result<u64, Response> {
        self.repo.delete(req).await
    }
}

impl<R> Integrations<R>
where
    R: Repository<IntegrationRequest, Model>,
    Self: EntityCache<Environments, Value = Model>,
{
    /* ===========================
     * START
     * ===========================
     */

    pub async fn start_integrations(&self, env: Environments) -> Result<(), Response> {
        self.set_state(env, LifecycleState::Starting)
            .await
            .map_err(Response::server_error)?;

        let mut req = IntegrationRequest::default();
        req.is_enabled = Some(true);

        let values = match self.repo.select_many(req, None).await {
            Ok(v) => v,
            Err(err) => {
                let _ = self.reset(env).await;
                return Err(err);
            }
        };

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

impl<R> Integrations<R>
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

#[cfg(test)]
mod core_tests {
    use models::structs::IntegrationRequest;

    use crate::{handler::Integrations, utils::MockRepo};

    #[tokio::test]
    async fn insert_integration_ok() {
        let repo = MockRepo::new();
        let service = Integrations::new(repo);

        let req = IntegrationRequest {
            id: Some(42),
            name: Some("Test".to_string()),
            code: Some("Test".to_string()),
            ..Default::default()
        };

        let result = service.insert(req).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, 42);
    }
}
