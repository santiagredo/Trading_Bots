use models::structs::Environments;

use crate::handler::Metrics;

pub struct LockSkipGuard;

impl LockSkipGuard {
    pub fn hit(environment: Environments) {
        tokio::spawn(async move {
            Metrics::set_skipped_metrics(environment).await;
        });
    }
}
