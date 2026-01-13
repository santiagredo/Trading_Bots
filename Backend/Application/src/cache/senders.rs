use std::{marker::PhantomData, sync::Arc};

use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{
    handler::Senders,
    utils::{Cache, Types},
};

static ACTIVE_SENDERS: Lazy<Arc<RwLock<Option<Senders>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Senders<Cache> {
    pub async fn set_senders_cache() -> Senders {
        let mut senders_lock = ACTIVE_SENDERS.write().await;

        let (command_sender, _) = Senders::set_commands_brodcast();
        let (event_sender, _) = Senders::set_events_broadcast();
        let (order_sender, _) = Senders::set_orders_broadcast();

        let senders = Senders {
            phase: PhantomData::<Types>,
            command_sender: command_sender.clone(),
            event_sender: event_sender.clone(),
            order_sender: order_sender.clone(),
        };

        *senders_lock = Some(senders.clone());

        senders
    }

    pub async fn get_senders_cache() -> Senders {
        let senders_lock = ACTIVE_SENDERS.read().await;

        if let Some(senders) = senders_lock.as_ref() {
            senders.clone()
        } else {
            drop(senders_lock);
            Self::set_senders_cache().await
        }
    }
}
