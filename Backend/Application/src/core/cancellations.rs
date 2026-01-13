use tokio_util::sync::CancellationToken;

use crate::{
    handler::{Cancellations, Engines},
    utils::{Cache, Core, Response},
};

impl Cancellations<Core> {
    async fn set_runtime_token_core(self, token: CancellationToken) {
        self.next_phase().set_runtime_token_cache(token).await
    }

    pub async fn get_runtime_token_core(&self) -> Option<CancellationToken> {
        Cancellations::<Cache>::get_runtime_token_cache(self.environment).await
    }

    pub async fn start_runtime_core(self) -> Result<CancellationToken, Response> {
        if Self::get_runtime_token_core(&self)
            .await
            .is_some_and(|token| !token.is_cancelled())
        {
            return Err(Response::bad_request(format!(
                "Active runtime cancellation token is already set"
            )));
        }

        let global_token = Engines::default().get_engine_token();
        let runtime_token = global_token.child_token();

        Self::set_runtime_token_core(self, runtime_token.clone()).await;

        Ok(runtime_token)
    }

    pub async fn stop_runtime_core(self) {
        self.next_phase().stop_runtime_cache().await
    }
}
