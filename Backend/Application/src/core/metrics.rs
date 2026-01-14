use std::time::Duration;

use models::{
    entities::critical_metrics::Model,
    structs::{CriticalMetric, Environments, QueryOptions},
};

use crate::{
    handler::{Metrics, DBC},
    utils::{Cache, Core, Response},
};

impl Metrics<Core> {
    // db
    pub async fn insert_metrics_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase()
            .insert_metrics_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_metrics_core(
        self,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase()
            .select_metrics_data(&DBC::db(&env).await?, query)
            .await
    }

    // cache
    pub async fn get_metric_core(self) -> Option<CriticalMetric> {
        self.next_phase().get_metrics_cache().await
    }

    pub async fn set_execution_metrics_core(
        environment: Environments,
        elapsed: Duration,
        success: bool,
    ) {
        Metrics::<Cache>::set_execution_metrics_cache(environment, elapsed, success).await
    }

    pub async fn set_posting_metrics_core(environment: Environments, increase: bool) {
        Metrics::<Cache>::set_posting_metrics_cache(environment, increase).await
    }

    pub async fn set_skipped_metrics_core(environment: Environments) {
        Metrics::<Cache>::set_skipped_metrics_cache(environment).await
    }

    pub async fn stop_metrics_core(self) {
        self.next_phase().stop_metrics_cache().await
    }

    // misc
    pub async fn persist_metrics_core(mut self) -> Result<Model, Response> {
        let env = self.environment;

        let critical_metric = Metrics::<Cache>::persist_metrics_cache(env)
            .await
            .unwrap_or_default();

        self.model = critical_metric;

        Self::insert_metrics_core(self).await
    }
}
