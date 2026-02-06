use crate::utils::Response;
use models::structs::Environments;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Default)]
pub struct Cancellations;

impl Cancellations {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_runtime_token(self, env: Environments) -> Option<CancellationToken> {
        self.get_runtime_token_core(env).await
    }

    pub async fn start_runtime(self, env: Environments) -> Result<CancellationToken, Response> {
        self.start_runtime_core(env).await
    }
}
