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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use models::structs::{CriticalMetric, Environments};

    use crate::{handler::Metrics, utils::Cache};

    // Helpers
    fn metrics(env: Environments) -> Metrics<Cache> {
        Metrics::default().with_env(env).next_phase()
    }

    async fn reset_env(env: Environments) {
        Metrics::default()
            .with_env(env)
            .next_phase()
            .stop_active_metrics_cache()
            .await;
        Metrics::<Cache>::persist_metrics_cache(env).await;
    }

    // Scenarios

    // Verifies initial state
    async fn scenario_initial_metrics_is_none(env: Environments) {
        reset_env(env).await;

        let result = metrics(env).get_active_metrics_cache().await;

        assert!(result.is_none());
    }

    // Verifies successful execution metrics
    async fn scenario_success_execution_updates_metrics(env: Environments) {
        reset_env(env).await;

        Metrics::<Cache>::set_active_execution_metrics_cache(env, Duration::from_millis(100), true)
            .await;

        let metric = metrics(env)
            .get_active_metrics_cache()
            .await
            .expect("metrics should exist");

        assert_eq!(metric.executions_ok, 1);
        assert_eq!(metric.executions_err, 0);
        assert_eq!(metric.consecutive_errors, 0);
        assert!(metric.last_success.is_some());
        assert_eq!(metric.total_execution_time, Duration::from_millis(100));
        assert_eq!(metric.max_execution_time, Duration::from_millis(100));
    }

    // Verifies error execution metrics
    async fn scenario_error_execution_updates_metrics(env: Environments) {
        reset_env(env).await;

        Metrics::<Cache>::set_active_execution_metrics_cache(env, Duration::from_millis(50), false)
            .await;

        let metric = metrics(env)
            .get_active_metrics_cache()
            .await
            .expect("metrics should exist");

        assert_eq!(metric.executions_ok, 0);
        assert_eq!(metric.executions_err, 1);
        assert_eq!(metric.consecutive_errors, 1);
        assert!(metric.last_error.is_some());
    }

    // Verifies max execution time logic
    async fn scenario_max_execution_time_is_tracked(env: Environments) {
        reset_env(env).await;

        Metrics::<Cache>::set_active_execution_metrics_cache(env, Duration::from_millis(100), true)
            .await;

        Metrics::<Cache>::set_active_execution_metrics_cache(env, Duration::from_millis(50), true)
            .await;

        let metric = metrics(env).get_active_metrics_cache().await.unwrap();

        assert_eq!(metric.max_execution_time, Duration::from_millis(100));
    }

    // Verifies posting metrics
    async fn scenario_active_posting_metrics(env: Environments) {
        reset_env(env).await;

        Metrics::<Cache>::set_active_posting_metrics_cache(env, true).await;
        Metrics::<Cache>::set_active_posting_metrics_cache(env, true).await;
        Metrics::<Cache>::set_active_posting_metrics_cache(env, false).await;

        let metric = metrics(env).get_active_metrics_cache().await.unwrap();

        assert_eq!(metric.active_posting, 1);
        assert_eq!(metric.max_active_posting, 2);
    }

    // Verifies skipped metrics
    async fn scenario_skipped_metrics(env: Environments) {
        reset_env(env).await;

        Metrics::<Cache>::set_active_skipped_metrics_cache(env).await;
        Metrics::<Cache>::set_active_skipped_metrics_cache(env).await;

        let metric = metrics(env).get_active_metrics_cache().await.unwrap();

        assert_eq!(metric.skipped_due_to_lock, 2);
    }

    // Verifies stop resets metrics
    async fn scenario_stop_resets_metrics(env: Environments) {
        reset_env(env).await;

        Metrics::<Cache>::set_active_execution_metrics_cache(env, Duration::from_millis(10), true)
            .await;

        metrics(env).stop_active_metrics_cache().await;

        let metric = metrics(env).get_active_metrics_cache().await.unwrap();

        assert_eq!(metric, CriticalMetric::default());
    }

    // Verifies persist removes metrics
    async fn scenario_persist_metrics(env: Environments) {
        reset_env(env).await;

        Metrics::<Cache>::set_active_execution_metrics_cache(env, Duration::from_millis(10), true)
            .await;

        let persisted = Metrics::<Cache>::persist_metrics_cache(env).await;
        assert!(persisted.is_some());

        let after = metrics(env).get_active_metrics_cache().await;
        assert!(after.is_none());
    }

    #[tokio::test]
    async fn cache_metrics_unit_responsibilities() {
        let env = Environments::DEV;

        scenario_initial_metrics_is_none(env).await;
        scenario_success_execution_updates_metrics(env).await;
        scenario_error_execution_updates_metrics(env).await;
        scenario_max_execution_time_is_tracked(env).await;
        scenario_active_posting_metrics(env).await;
        scenario_skipped_metrics(env).await;
        scenario_stop_resets_metrics(env).await;
        scenario_persist_metrics(env).await;

        reset_env(env).await;
    }
}
