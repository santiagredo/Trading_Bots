use crate::utils::{Core, Types};
use models::{enums::MetricType, structs::Metric};
use sea_orm::prelude::DateTime;
use std::{collections::HashMap, marker::PhantomData, time::Duration};

#[derive(Debug, Default)]
pub struct Metrics<Phase = Types> {
    phase: PhantomData<Phase>,
    pub model: Metric,
}

impl Metrics {
    pub async fn set_active_metrics(metrics: Option<Vec<Metric>>) -> Option<Vec<Metric>> {
        Metrics::<Core>::set_active_metrics_core(metrics).await
    }

    pub async fn set_active_metric(
        metric_type: MetricType,
        elapsed: Duration,
        date_time: DateTime,
    ) {
        Metrics::<Core>::set_active_metric_core(metric_type, elapsed, date_time).await
    }

    pub async fn get_active_metrics() -> Option<HashMap<MetricType, Metric>> {
        Metrics::<Core>::get_active_metrics_core().await
    }

    pub async fn get_active_metric(key: &MetricType) -> Option<Metric> {
        Metrics::<Core>::get_active_metric_core(key).await
    }

    pub async fn start_active_metrics() {
        Metrics::<Core>::start_active_metrics_core().await
    }

    pub async fn stop_active_metrics() {
        Metrics::<Core>::stop_active_metrics_core().await
    }
}
