use crate::{
    handler::{Integrations, IntegrationsSettings},
    logic,
    utils::{AnyRepo, EntityCache, Repository, Response, Select},
};
use models::{
    entities::{self, integration_settings::Model},
    enums::{FiniteStateMachine, LifecycleState},
    structs::{CacheIntegrationsSettings, Environments, IntegrationSettingRequest, QueryOptions},
};

/* ======================================================
 * CRUD / DB
 * ======================================================
 */

impl<R> IntegrationsSettings<R>
where
    R: Repository<IntegrationSettingRequest, Model>,
{
    pub async fn insert(&self, req: IntegrationSettingRequest) -> Result<Model, Response> {
        self.repo.insert(req).await
    }

    pub async fn select(&self, req: IntegrationSettingRequest) -> Result<Option<Model>, Response> {
        self.repo.select(req).await
    }

    pub async fn select_many(
        &self,
        req: IntegrationSettingRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        self.repo.select_many(req, query).await
    }

    pub async fn update(&self, req: IntegrationSettingRequest) -> Result<Model, Response> {
        logic::integrations_settings::validate_update(&req).map_err(Response::bad_request)?;
        self.repo.update(req).await
    }

    pub async fn delete(&self, req: IntegrationSettingRequest) -> Result<u64, Response> {
        self.repo.delete(req).await
    }
}

impl IntegrationsSettings<AnyRepo<IntegrationSettingRequest, Model>>
where
    Self: EntityCache<Environments>,
{
    /* ===========================
     * START ACTIVE INTEGRATION SETTINGS
     * ===========================
     */

    pub async fn start(&self, environment: Environments) -> Result<(), Response> {
        // STARTING
        self.set_state(environment, LifecycleState::Starting)
            .await
            .map_err(Response::server_error)?;

        /*
         * Load active integrations FROM CACHE (in-memory)
         */

        let active_integrations = Integrations::blank()
            .get_all(environment)
            .await
            .ok_or_else(|| {
                Response::server_error("Integrations cache not initialized".to_owned())
            })?;

        if !active_integrations.status.allows(LifecycleState::Running) {
            let _ = self
                .reset(environment)
                .await
                .map_err(Response::server_error)?;

            return Err(Response::server_error(
                "Integrations cache not running".to_owned(),
            ));
        }

        let active_integration_ids = active_integrations
            .models
            .keys()
            .copied()
            .collect::<std::collections::HashSet<_>>();

        /*
         * Load active settings FROM DB
         */

        let req = IntegrationSettingRequest::default();

        let values = match self.repo.select_many(req, None).await {
            Ok(s) => s
                .into_iter()
                .filter(|s| active_integration_ids.contains(&s.integration_id))
                .collect::<Vec<_>>(),
            Err(err) => {
                let _ = self
                    .reset(environment)
                    .await
                    .map_err(Response::server_error)?;

                return Err(err);
            }
        };

        // RUNNING
        self.set_state(environment, LifecycleState::Running)
            .await
            .map_err(Response::server_error)?;

        // Populate cache
        if let Err(err) = self.set_all(environment, values).await {
            let _ = self.reset(environment).await;
            return Err(Response::server_error(err));
        }

        Ok(())
    }
}

impl<R> IntegrationsSettings<R>
where
    Self: EntityCache<Environments>,
{
    /* ===========================
     * STOP ACTIVE INTEGRATION SETTINGS
     * ===========================
     */

    pub async fn stop(&self, environment: Environments) -> Result<(), Response> {
        self.set_state(environment, LifecycleState::Stopping)
            .await
            .map_err(Response::server_error)?;

        self.remove_all(environment)
            .await
            .map_err(Response::server_error)?;

        self.set_state(environment, LifecycleState::Off)
            .await
            .map_err(Response::server_error)?;

        Ok(())
    }

    /* ===========================
     * RESET ACTIVE INTEGRATION SETTINGS
     * ===========================
     */
}

impl<R> IntegrationsSettings<R>
where
    R: Repository<IntegrationSettingRequest, Model>,
    Self: EntityCache<Environments, Collection = CacheIntegrationsSettings>,
{
    // /* ===========================
    //  * RESOLVERS
    //  * ===========================
    //  */
    pub async fn resolve_integration_settings(
        &self,
        environment: Environments,
        req: IntegrationSettingRequest,
    ) -> Result<Vec<entities::integration_settings::Model>, Response> {
        let cache_settings = match self.get_all(environment).await {
            None => None,
            Some(val) => val
                .integrations_map
                .get(&req.integration_id.unwrap_or_default())
                .map(|map| map.models.values().cloned().collect::<Vec<Model>>()),
        };

        if let Some(settings) = cache_settings {
            return Ok(settings);
        }

        self.select_many(req, None).await
    }
}
