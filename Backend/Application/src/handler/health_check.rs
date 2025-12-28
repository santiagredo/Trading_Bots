use std::{collections::BTreeMap, marker::PhantomData};

use crate::utils::{Response, Types};

pub struct HealthCheck<Phase = Types> {
    phase: PhantomData<Phase>,
}

impl HealthCheck {
    pub async fn select_health_check() -> Result<BTreeMap<String, bool>, Response> {
        HealthCheck::select_health_check_core().await
    }
}
