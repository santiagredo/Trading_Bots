use std::marker::PhantomData;

use models::{enums::WebsocketCommand, structs::Ticker};
use tokio::sync::broadcast::{self, Receiver, Sender};

use crate::utils::{Core, Types};

#[derive(Debug, Clone)]
pub struct Senders<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub command_sender: broadcast::Sender<WebsocketCommand>,
    pub event_sender: broadcast::Sender<Ticker>,
}

impl Senders {
    pub fn set_commands_brodcast() -> (Sender<WebsocketCommand>, Receiver<WebsocketCommand>) {
        broadcast::channel::<WebsocketCommand>(64)
    }

    pub fn set_events_broadcast() -> (Sender<Ticker>, Receiver<Ticker>) {
        broadcast::channel::<Ticker>(64)
    }

    // async fn set_active_senders() -> Senders {
    //     Senders::<Core>::set_active_senders_core().await
    // }

    pub async fn get_active_senders() -> Senders {
        Senders::<Core>::get_active_senders_core().await
    }
}
