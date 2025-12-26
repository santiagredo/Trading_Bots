use std::marker::PhantomData;

use models::{
    entities::orders,
    enums::WebsocketCommand,
    structs::{Environments, Ticker},
};
use tokio::sync::broadcast::{self, Receiver, Sender};

use crate::utils::{Core, Types};

#[derive(Debug, Clone)]
pub struct Senders<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub command_sender: broadcast::Sender<WebsocketCommand>,
    pub event_sender: broadcast::Sender<Ticker>,
    pub order_sender: broadcast::Sender<(Environments, orders::Model)>,
}

impl Senders {
    pub fn set_commands_brodcast() -> (Sender<WebsocketCommand>, Receiver<WebsocketCommand>) {
        broadcast::channel::<WebsocketCommand>(64)
    }

    pub fn set_events_broadcast() -> (Sender<Ticker>, Receiver<Ticker>) {
        broadcast::channel::<Ticker>(64)
    }

    pub fn set_orders_broadcast() -> (
        Sender<(Environments, orders::Model)>,
        Receiver<(Environments, orders::Model)>,
    ) {
        broadcast::channel::<(Environments, orders::Model)>(64)
    }

    pub async fn get_active_senders() -> Senders {
        Senders::<Core>::get_active_senders_core().await
    }
}
