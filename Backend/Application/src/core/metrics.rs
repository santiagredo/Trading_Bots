use std::{collections::HashMap, time::Duration};

use models::{enums::MetricType, structs::Metric};
use sea_orm::prelude::DateTime;

use crate::{
    handler::Metrics,
    utils::{Cache, Core},
};

impl Metrics<Core> {
    pub async fn set_active_metrics_core(metrics: Option<Vec<Metric>>) -> Option<Vec<Metric>> {
        Metrics::<Cache>::set_active_metrics_cache(metrics).await
    }

    pub async fn set_active_metric_core(
        metric_type: MetricType,
        elapsed: Duration,
        date_time: DateTime,
    ) {
        Metrics::<Cache>::set_active_metric_cache(metric_type, elapsed, date_time).await
    }

    pub async fn get_active_metrics_core() -> Option<HashMap<MetricType, Metric>> {
        Metrics::<Cache>::get_active_metrics_cache().await
    }

    pub async fn get_active_metric_core(key: &MetricType) -> Option<Metric> {
        Metrics::<Cache>::get_active_metric_cache(key).await
    }

    pub async fn start_active_metrics_core() {
        Metrics::<Cache>::start_active_metrics_cache().await
    }

    pub async fn stop_active_metrics_core() {
        Metrics::<Cache>::stop_active_metrics_cache().await
    }
}
