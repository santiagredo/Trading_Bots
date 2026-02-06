use crate::{
    handler::{Cancellations, Engines},
    utils::Response,
};
use models::structs::Environments;
use tokio_util::sync::CancellationToken;

impl Cancellations {
    async fn set_runtime_token_core(self, environment: Environments, token: CancellationToken) {
        self.set_runtime_token_cache(environment, token).await
    }

    pub async fn get_runtime_token_core(&self, env: Environments) -> Option<CancellationToken> {
        Cancellations::get_runtime_token_cache(env).await
    }

    pub async fn start_runtime_core(
        self,
        env: Environments,
    ) -> Result<CancellationToken, Response> {
        if Self::get_runtime_token_core(&self, env)
            .await
            .is_some_and(|token| !token.is_cancelled())
        {
            return Err(Response::bad_request(format!(
                "Active runtime cancellation token is already set"
            )));
        }

        let global_token = Engines::blank().get_engine_token_core();
        let runtime_token = global_token.child_token();

        Self::set_runtime_token_core(self, env, runtime_token.clone()).await;

        Ok(runtime_token)
    }

    pub async fn stop_runtime_core(self, env: Environments) {
        self.stop_runtime_cache(env).await
    }
}
