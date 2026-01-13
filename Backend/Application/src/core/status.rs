use std::collections::HashMap;

use models::{
    entities::status::Model,
    enums::{LifecycleState, Status},
};

use crate::{
    handler::{OrderStatus, DBC},
    utils::{Cache, Core, Data, Response},
};

impl OrderStatus<Core> {
    /* ===========================
     * DB
     * ===========================
     */

    pub async fn select_status_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_status_data(&DBC::db(&env).await?)
            .await
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_statuses_core(self) -> Option<HashMap<i32, Model>> {
        OrderStatus::<Cache>::get_status_cache(self.environment)
            .await
            .map(|c| c.models)
    }

    pub async fn get_status_by_enum_core(self, status: Status) -> Option<Model> {
        OrderStatus::<Cache>::get_status_by_enum(self.environment, status).await
    }

    pub async fn get_status_state_core(self) -> LifecycleState {
        OrderStatus::<Cache>::get_cache_state(self.environment).await
    }

    /* ===========================
     * START ACTIVE STATUS
     * ===========================
     */

    pub async fn start_status_core(self) -> Result<(), Response> {
        let env = self.environment;

        // =========================
        // STARTING
        // =========================
        if let Err(err) =
            OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Starting).await
        {
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // =========================
        // Load statuses from DB
        // =========================
        let status_request = OrderStatus::default().with_env(env);

        let statuses = match status_request.select_status().await {
            Ok(s) => s,
            Err(err) => {
                let _ = OrderStatus::<Cache>::reset_status_cache(env).await;
                return Err(err);
            }
        };

        // =========================
        // RUNNING
        // =========================
        if let Err(err) = OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Running).await
        {
            let _ = OrderStatus::<Cache>::reset_status_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // =========================
        // Populate cache
        // =========================
        if let Err(err) = OrderStatus::<Cache>::set_statuses_cache(env, statuses).await {
            let _ = OrderStatus::<Cache>::reset_status_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    /* ===========================
     * STOP ACTIVE STATUS
     * ===========================
     */

    pub async fn stop_status_core(self) -> Result<(), Response> {
        let env = self.environment;

        // =========================
        // STOPPING
        // =========================
        if let Err(err) =
            OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Stopping).await
        {
            let _ = OrderStatus::<Cache>::reset_status_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // =========================
        // Remove statuses
        // =========================
        if let Err(err) = OrderStatus::<Cache>::remove_statuses_cache(env).await {
            let _ = OrderStatus::<Cache>::reset_status_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // =========================
        // OFF
        // =========================
        if let Err(err) = OrderStatus::<Cache>::set_status_cache(env, LifecycleState::Off).await {
            let _ = OrderStatus::<Cache>::reset_status_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    pub async fn reset_status_core(self) -> Result<(), Response> {
        OrderStatus::<Cache>::reset_status_cache(self.environment)
            .await
            .map_err(Response::server_error)
    }
}
