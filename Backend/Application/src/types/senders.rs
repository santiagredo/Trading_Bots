use std::{marker::PhantomData, sync::Arc};

use models::{enums::WebsocketCommand, structs::Ticker};
use once_cell::sync::Lazy;
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    RwLock,
};

use crate::utils::Types;

#[derive(Debug, Clone)]
pub struct Senders<Phase = Types> {
    phase: PhantomData<Phase>,
    pub command_sender: broadcast::Sender<WebsocketCommand>,
    pub event_sender: broadcast::Sender<Ticker>,
    // pub indicator_sender: broadcast::Sender<(i32, String)>,
}

static ACTIVE_SENDERS: Lazy<Arc<RwLock<Option<Senders>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Senders {
    fn set_commands_brodcast() -> (Sender<WebsocketCommand>, Receiver<WebsocketCommand>) {
        broadcast::channel::<WebsocketCommand>(64)
    }

    fn set_events_broadcast() -> (Sender<Ticker>, Receiver<Ticker>) {
        broadcast::channel::<Ticker>(64)
    }

    // fn set_indicators_broadcast() -> (Sender<(i32, String)>, Receiver<(i32, String)>) {
    //     broadcast::channel::<(i32, String)>(64)
    // }

    async fn set_active_senders() -> Senders {
        // let mut ws_handle_write = BINANCE_WS_ACTIVE_SENDERS.write().await;
        // *ws_handle_write = None;
        // drop(ws_handle_write);

        let mut senders_lock = ACTIVE_SENDERS.write().await;

        let (command_sender, _) = Self::set_commands_brodcast();
        let (event_sender, _) = Self::set_events_broadcast();
        // let (indicator_sender, _) = Self::set_indicators_broadcast();

        let senders = Senders {
            phase: PhantomData::<Types>,
            command_sender: command_sender.clone(),
            event_sender: event_sender.clone(),
            // indicator_sender: indicator_sender.clone(),
        };

        *senders_lock = Some(senders.clone());

        senders
    }

    pub async fn get_active_senders() -> Senders {
        let senders_lock = ACTIVE_SENDERS.read().await;

        if let Some(senders) = senders_lock.as_ref() {
            senders.clone()
        } else {
            drop(senders_lock);
            Self::set_active_senders().await
        }
    }
}
