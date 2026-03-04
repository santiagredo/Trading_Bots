use std::collections::HashMap;

use tokio_util::sync::CancellationToken;

use crate::models::structs::Environments;

#[derive(Default)]
pub struct CacheRuntimeTokens {
    pub environment_tokens: HashMap<Environments, CancellationToken>,
}
