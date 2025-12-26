use std::{collections::HashMap, sync::Arc, time::Duration};

use models::structs::{CriticalMetric, Environments};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{handler::Metrics, utils::Cache};

#[derive(Default)]
struct CacheEnvironments {
    pub environments: HashMap<Environments, CacheMetrics>,
}

#[derive(Default)]
struct CacheMetrics {
    pub model: CriticalMetric,
}

static ACTIVE_METRICS: Lazy<Arc<RwLock<CacheEnvironments>>> =
    Lazy::new(|| Arc::new(RwLock::new(CacheEnvironments::default())));

impl Metrics<Cache> {
    pub async fn get_active_metrics_cache(self) -> Option<CriticalMetric> {
        let environment = self.environment;

        let active_metrics = ACTIVE_METRICS.read().await;

        let env_map = active_metrics.environments.get(&environment)?;

        Some(env_map.model.clone())
    }

    pub async fn set_active_execution_metrics_cache(
        environment: Environments,
        elapsed: Duration,
        success: bool,
    ) {
        let mut active_metrics = ACTIVE_METRICS.write().await;

        let env_map = active_metrics
            .environments
            .entry(environment)
            .or_insert_with(CacheMetrics::default);

        let now = chrono::Local::now().naive_local();

        if success {
            env_map.model.executions_ok += 1;
            env_map.model.consecutive_errors = 0;
            env_map.model.last_success = Some(now);
        } else {
            env_map.model.executions_err += 1;
            env_map.model.consecutive_errors += 1;
            env_map.model.last_error = Some(now);
        }

        env_map.model.total_execution_time += elapsed;

        if elapsed > env_map.model.max_execution_time {
            env_map.model.max_execution_time = elapsed;
        }
    }

    pub async fn set_active_posting_metrics_cache(environment: Environments, increase: bool) {
        let mut active_metrics = ACTIVE_METRICS.write().await;

        let env_map = active_metrics
            .environments
            .entry(environment)
            .or_insert_with(CacheMetrics::default);

        if increase {
            env_map.model.active_posting += 1;
            env_map.model.max_active_posting = env_map
                .model
                .max_active_posting
                .max(env_map.model.active_posting);

            return;
        }

        if env_map.model.active_posting > 0 {
            env_map.model.active_posting -= 1
        }
    }

    pub async fn set_active_skipped_metrics_cache(environment: Environments) {
        let mut active_metrics = ACTIVE_METRICS.write().await;

        let env_map = active_metrics
            .environments
            .entry(environment)
            .or_insert_with(CacheMetrics::default);

        env_map.model.skipped_due_to_lock += 1;
    }

    pub async fn persist_metrics_cache(environment: Environments) -> Option<CriticalMetric> {
        let mut active_metrics = ACTIVE_METRICS.write().await;

        let env_map = active_metrics.environments.remove(&environment)?;

        Some(env_map.model.clone())
    }

    pub async fn stop_active_metrics_cache(self) {
        let environment = self.environment;

        let mut active_metrics = ACTIVE_METRICS.write().await;

        if let Some(env_map) = active_metrics.environments.get_mut(&environment) {
            env_map.model = CriticalMetric::default()
        };
    }
}
