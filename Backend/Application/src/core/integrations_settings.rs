use std::collections::HashMap;

use models::{
    entities::{self, integration_settings::Model},
    enums::{FiniteStateMachine, LifecycleState},
    structs::CacheIntegrationsSettings,
};

use crate::{
    handler::{Integrations, IntegrationsSettings, DBC},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl IntegrationsSettings<Core> {
    /* ===========================
     * DB
     * ===========================
     */

    pub async fn select_integrations_settings_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_integrations_settings_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn update_integration_setting_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase()
            .update_integration_setting_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .update_integration_setting_data(&DBC::db(&env).await?)
            .await
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_integrations_settings_core(self) -> Option<CacheIntegrationsSettings> {
        IntegrationsSettings::<Cache>::get_integrations_settings_cache(self.environment).await
    }

    pub async fn get_integration_settings_core(self) -> Option<HashMap<i32, Model>> {
        IntegrationsSettings::<Cache>::get_integration_settings_cache(
            self.environment,
            self.model.integration_id.unwrap_or_default(),
        )
        .await
    }

    pub async fn get_integration_setting_core(self) -> Option<Model> {
        let integration_id = self.model.integration_id.unwrap_or_default();

        IntegrationsSettings::<Cache>::get_integration_setting_cache(
            self.environment,
            integration_id,
        )
        .await
    }

    pub async fn get_integrations_settings_state_core(self) -> LifecycleState {
        IntegrationsSettings::<Cache>::get_cache_state(self.environment).await
    }

    /* ===========================
     * START ACTIVE INTEGRATION SETTINGS
     * ===========================
     */

    pub async fn start_integrations_settings_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STARTING
        if let Err(err) =
            IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Starting).await
        {
            return Err(Response::server_error(err));
        }

        /*
         * Load active integrations FROM CACHE (in-memory)
         */
        let active_integrations = Integrations::<Cache>::get_integrations_cache(env)
            .await
            .ok_or_else(|| {
                Response::server_error("Integrations cache not initialized".to_owned())
            })?;

        if !active_integrations.status.allows(LifecycleState::Running) {
            let _ = IntegrationsSettings::<Cache>::reset_integrations_settings_cache(env).await;
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
        let settings_request = IntegrationsSettings::default().with_env(env);

        let settings = match settings_request.select_integrations_settings().await {
            Ok(s) => s
                .into_iter()
                .filter(|s| active_integration_ids.contains(&s.integration_id))
                .collect::<Vec<_>>(),
            Err(err) => {
                let _ = IntegrationsSettings::<Cache>::reset_integrations_settings_cache(env).await;
                return Err(err);
            }
        };

        // RUNNING
        if let Err(err) =
            IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Running).await
        {
            let _ = IntegrationsSettings::<Cache>::reset_integrations_settings_cache(env).await;
            return Err(Response::server_error(err));
        }

        // Populate cache
        if let Err(err) =
            IntegrationsSettings::<Cache>::set_integrations_settings_cache(env, settings).await
        {
            let _ = IntegrationsSettings::<Cache>::reset_integrations_settings_cache(env).await;
            return Err(Response::server_error(err));
        }

        Ok(())
    }

    /* ===========================
     * STOP ACTIVE INTEGRATION SETTINGS
     * ===========================
     */

    pub async fn stop_integrations_settings_core(self) -> Result<(), Response> {
        let env = self.environment;

        // STOPPING
        if let Err(err) =
            IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Stopping).await
        {
            let _ = IntegrationsSettings::<Cache>::reset_integrations_settings_cache(env).await;
            return Err(Response::server_error(err));
        }

        // Remove settings
        if let Err(err) =
            IntegrationsSettings::<Cache>::remove_integrations_settings_cache(env).await
        {
            let _ = IntegrationsSettings::<Cache>::reset_integrations_settings_cache(env).await;
            return Err(Response::server_error(err));
        }

        // OFF
        if let Err(err) =
            IntegrationsSettings::<Cache>::set_status_cache(env, LifecycleState::Off).await
        {
            let _ = IntegrationsSettings::<Cache>::reset_integrations_settings_cache(env).await;
            return Err(Response::server_error(err));
        }

        Ok(())
    }

    /* ===========================
     * RESET INTEGRATION SETTINGS
     * ===========================
     */

    pub async fn reset_integrations_settings_core(self) -> Result<(), Response> {
        IntegrationsSettings::<Cache>::reset_integrations_settings_cache(self.environment)
            .await
            .map_err(Response::server_error)
    }

    /* ===========================
     * RESOLVERS
     * ===========================
     */

    pub async fn resolve_integration_settings_core(
        &self,
    ) -> Result<Vec<entities::integration_settings::Model>, Response> {
        let mut req = IntegrationsSettings::default().with_env(self.environment);
        req.model.integration_id = Some(self.model.integration_id.unwrap_or_default());

        let cache_settings = IntegrationsSettings::default()
            .with_env(self.environment)
            .get_integration_settings()
            .await;

        match cache_settings {
            Some(val) => Ok(val.values().cloned().collect()),
            None => req.select_integrations_settings().await,
        }
    }

    pub fn resolve_setting_value_core(
        settings: &[entities::integration_settings::Model],
        nick: &str,
    ) -> Result<String, Response> {
        IntegrationsSettings::<Logic>::resolve_setting_value_logic(settings, nick)
            .map_err(Response::not_found)
    }
}
