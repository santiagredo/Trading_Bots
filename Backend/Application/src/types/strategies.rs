use std::{marker::PhantomData, sync::Arc};

use models::entities::strategies::Model;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::utils::Core;

#[derive(Debug, Default)]
pub struct Strategies<Phase = Core> {
    pub phase: PhantomData<Phase>,
    pub model: Model,
}

pub static STRATEGIES: Lazy<Arc<RwLock<Vec<Model>>>> =
    Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

impl Strategies {
    pub async fn get_active_strategies() -> Vec<Model> {
        let mut strategies = STRATEGIES.write().await;

        if strategies.is_empty() {
            let active_strategies = Strategies::<Core>::select_active_strategies()
                .await
                .unwrap_or_default();

            *strategies = active_strategies;

            println!("strategies: {strategies:?}");
        }

        strategies.clone()
    }

    pub async fn reload_active_strategies() {
        let mut strategies = STRATEGIES.write().await;

        let active_strategies = Strategies::<Core>::select_active_strategies()
            .await
            .unwrap_or_default();

        *strategies = active_strategies;

        println!("strategies: {strategies:?}");
    }
}
