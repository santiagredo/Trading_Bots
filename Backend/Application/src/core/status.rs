use crate::{
    handler::{OrderStatus, DBC},
    utils::Response,
};
use models::{entities::status::Model, enums::LifecycleState, structs::Environments};
use std::collections::HashMap;

impl OrderStatus {
    /* ===========================
     * DB
     * ===========================
     */

    pub async fn select(env: Environments) -> Result<Vec<Model>, Response> {
        Self::select_status_data(&DBC::db(&env).await?).await
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_statuses(self, env: Environments) -> Option<HashMap<i32, Model>> {
        OrderStatus::get_status_cache(env).await.map(|c| c.models)
    }

    pub async fn get_status_state(self, env: Environments) -> LifecycleState {
        OrderStatus::get_cache_state(env).await
    }

    /* ===========================
     * START ACTIVE STATUS
     * ===========================
     */

    pub async fn start_status(self, env: Environments) -> Result<(), Response> {
        // =========================
        // STARTING
        // =========================
        if let Err(err) = OrderStatus::set_status_cache(env, LifecycleState::Starting).await {
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // =========================
        // Load statuses from DB
        // =========================

        let statuses = match Self::select(env).await {
            Ok(s) => s,
            Err(err) => {
                let _ = OrderStatus::reset_status_cache(env).await;
                return Err(err);
            }
        };

        // =========================
        // RUNNING
        // =========================
        if let Err(err) = OrderStatus::set_status_cache(env, LifecycleState::Running).await {
            let _ = OrderStatus::reset_status_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // =========================
        // Populate cache
        // =========================
        if let Err(err) = OrderStatus::set_statuses_cache(env, statuses).await {
            let _ = OrderStatus::reset_status_cache(env).await;
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

    pub async fn stop_status(self, env: Environments) -> Result<(), Response> {
        // =========================
        // STOPPING
        // =========================
        if let Err(err) = OrderStatus::set_status_cache(env, LifecycleState::Stopping).await {
            let _ = OrderStatus::reset_status_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // =========================
        // Remove statuses
        // =========================
        if let Err(err) = OrderStatus::remove_statuses_cache(env).await {
            let _ = OrderStatus::reset_status_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        // =========================
        // OFF
        // =========================
        if let Err(err) = OrderStatus::set_status_cache(env, LifecycleState::Off).await {
            let _ = OrderStatus::reset_status_cache(env).await;
            return Err(Response {
                code: 500,
                message: err,
            });
        }

        Ok(())
    }

    pub async fn reset_status(self, env: Environments) -> Result<(), Response> {
        OrderStatus::reset_status_cache(env)
            .await
            .map_err(Response::server_error)
    }
}
