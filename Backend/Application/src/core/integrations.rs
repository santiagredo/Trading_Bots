use models::{entities::integrations::Model, enums::LifecycleState, structs::CacheIntegrations};

use crate::{
    handler::{Integrations, DBC},
    utils::{Cache, Core, Data, Response},
};

impl Integrations<Core> {
    /* ===========================
     * DB
     * ===========================
     */

    pub async fn select_integrations_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_integrations_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn update_integration_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .update_integration_data(&DBC::db(&env).await?)
            .await
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_integrations_core(self) -> Option<CacheIntegrations> {
        Integrations::<Cache>::get_integrations_cache(self.environment).await
    }

    pub async fn get_integration_core(self) -> Option<Model> {
        let id = self.model.id.unwrap_or_default();

        Integrations::<Cache>::get_integration_cache(self.environment, id).await
    }

    pub async fn get_integrations_state_core(self) -> LifecycleState {
        Integrations::<Cache>::get_cache_state(self.environment).await
    }

    /* ===========================
     * START ACTIVE INTEGRATIONS
     * ===========================
     */

    pub async fn start_integrations_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STARTING
        if let Err(err) =
            Integrations::<Cache>::set_status_cache(env, LifecycleState::Starting).await
        {
            return Err(Response::server_error(err));
        }

        // Load active integrations from DB
        let mut integrations_request = Integrations::default().with_env(env);
        integrations_request.model.is_enabled = Some(true);

        let integrations = match integrations_request.select_integrations().await {
            Ok(i) => i,
            Err(err) => {
                let _ = Integrations::<Cache>::reset_integrations_cache(env).await;
                return Err(err);
            }
        };

        // RUNNING
        if let Err(err) =
            Integrations::<Cache>::set_status_cache(env, LifecycleState::Running).await
        {
            let _ = Integrations::<Cache>::reset_integrations_cache(env).await;
            return Err(Response::server_error(err));
        }

        // Populate cache
        if let Err(err) = Integrations::<Cache>::set_integrations_cache(env, integrations).await {
            let _ = Integrations::<Cache>::reset_integrations_cache(env).await;
            return Err(Response::server_error(err));
        }

        Ok(())
    }

    /* ===========================
     * STOP ACTIVE INTEGRATIONS
     * ===========================
     */

    pub async fn stop_integrations_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STOPPING
        if let Err(err) =
            Integrations::<Cache>::set_status_cache(env, LifecycleState::Stopping).await
        {
            let _ = Integrations::<Cache>::reset_integrations_cache(env).await;
            return Err(Response::server_error(err));
        }

        // Remove integrations
        if let Err(err) = Integrations::<Cache>::remove_integrations_cache(env).await {
            let _ = Integrations::<Cache>::reset_integrations_cache(env).await;
            return Err(Response::server_error(err));
        }

        // OFF
        if let Err(err) = Integrations::<Cache>::set_status_cache(env, LifecycleState::Off).await {
            let _ = Integrations::<Cache>::reset_integrations_cache(env).await;
            return Err(Response::server_error(err));
        }

        Ok(())
    }

    /* ===========================
     * RESET INTEGRATIONS
     * ===========================
     */

    pub async fn reset_integrations_core(self) -> Result<(), Response> {
        Integrations::<Cache>::reset_integrations_cache(self.environment)
            .await
            .map_err(Response::server_error)
    }
}
