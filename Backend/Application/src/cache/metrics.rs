use std::{collections::HashMap, sync::Arc, time::Duration};

use models::{enums::MetricType, structs::Metric};
use once_cell::sync::Lazy;
use sea_orm::prelude::DateTime;
use tokio::sync::RwLock;

use crate::{handler::Metrics, utils::Cache};

static ACTIVE_METRICS: Lazy<Arc<RwLock<Option<HashMap<MetricType, Metric>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Metrics<Cache> {
    pub async fn set_active_metrics_cache(metrics: Option<Vec<Metric>>) -> Option<Vec<Metric>> {
        let mut active_metrics = ACTIVE_METRICS.write().await;

        let Some(metrics) = metrics else {
            *active_metrics = None;
            return None;
        };

        let mut active_metrics_map: HashMap<MetricType, Metric> = HashMap::new();

        for metric in metrics.iter() {
            active_metrics_map.insert(metric.metric_type, metric.clone());
        }

        *active_metrics = Some(active_metrics_map);

        Some(metrics)
    }

    pub async fn set_active_metric_cache(
        metric_type: MetricType,
        elapsed: Duration,
        date_time: DateTime,
    ) {
        let mut active_metrics = ACTIVE_METRICS.write().await;

        let Some(metrics_map) = active_metrics.as_mut() else {
            return;
        };

        let Some(active_metric) = metrics_map.get_mut(&metric_type) else {
            return;
        };

        active_metric.add(elapsed, date_time);
    }

    pub async fn get_active_metrics_cache() -> Option<HashMap<MetricType, Metric>> {
        let active_metrics = ACTIVE_METRICS.read().await;

        active_metrics.clone()
    }

    pub async fn get_active_metric_cache(key: &MetricType) -> Option<Metric> {
        let active_metrics = ACTIVE_METRICS.read().await;

        let Some(active_metrics) = active_metrics.as_ref() else {
            return None;
        };

        active_metrics.get(key).cloned()
    }

    pub async fn start_active_metrics_cache() {
        if Self::get_active_metrics_cache()
            .await
            .is_none_or(|map| map.is_empty())
        {
            let mut all_metrics = Metric::default();
            all_metrics.metric_type = MetricType::All;

            let mut completed_metrics = Metric::default();
            completed_metrics.metric_type = MetricType::Completed;

            let metrics_vec = vec![all_metrics, completed_metrics];

            Self::set_active_metrics_cache(Some(metrics_vec)).await;
        }
    }

    pub async fn stop_active_metrics_cache() {
        Self::set_active_metrics_cache(None).await;
    }
}
