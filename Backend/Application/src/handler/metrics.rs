use crate::utils::{Core, Response, Types};
use models::{
    entities::critical_metrics::Model,
    structs::{CriticalMetric, Environments},
};
use std::{marker::PhantomData, time::Duration};

#[derive(Debug, Default)]
pub struct Metrics<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
    pub model: CriticalMetric,
}

impl<Phase> Metrics<Phase> {
    pub fn next_phase<Next>(self) -> Metrics<Next> {
        Metrics {
            phase: PhantomData::<Next>,
            environment: self.environment,
            model: self.model,
        }
    }
}

impl Metrics {
    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
            model: CriticalMetric::default(),
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: self.phase,
            environment,
            model: self.model,
        }
    }

    pub async fn get_active_metric(self) -> Option<CriticalMetric> {
        self.next_phase().get_active_metric_core().await
    }

    // cache
    pub async fn set_active_execution_metrics(
        environment: Environments,
        elapsed: Duration,
        success: bool,
    ) {
        Metrics::<Core>::set_active_execution_metrics_core(environment, elapsed, success).await
    }

    pub async fn set_active_posting_metrics(environment: Environments, increase: bool) {
        Metrics::<Core>::set_active_posting_metrics_core(environment, increase).await
    }

    pub async fn set_active_skipped_metrics(environment: Environments) {
        Metrics::<Core>::set_active_skipped_metrics_core(environment).await
    }

    pub async fn stop_active_metrics(self) {
        self.next_phase().stop_active_metrics_core().await
    }

    // misc
    pub async fn persist_metrics(self) -> Result<Model, Response> {
        self.next_phase().persist_metrics_core().await
    }
}
