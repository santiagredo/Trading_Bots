use models::{
    entities::orders,
    enums::WebsocketCommand,
    structs::{Environments, Ticker},
};
use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(Debug, Clone)]
pub struct Senders {
    pub command_sender: broadcast::Sender<WebsocketCommand>,
    pub event_sender: broadcast::Sender<Ticker>,
    pub order_sender: broadcast::Sender<(Environments, orders::Model)>,
}

impl Senders {
    pub fn new() -> Self {
        let (command_sender, _) = Self::set_commands_brodcast();
        let (event_sender, _) = Self::set_events_broadcast();
        let (order_sender, _) = Self::set_orders_broadcast();

        Self {
            command_sender,
            event_sender,
            order_sender,
        }
    }

    fn set_commands_brodcast() -> (Sender<WebsocketCommand>, Receiver<WebsocketCommand>) {
        broadcast::channel::<WebsocketCommand>(64)
    }

    fn set_events_broadcast() -> (Sender<Ticker>, Receiver<Ticker>) {
        broadcast::channel::<Ticker>(64)
    }

    fn set_orders_broadcast() -> (
        Sender<(Environments, orders::Model)>,
        Receiver<(Environments, orders::Model)>,
    ) {
        broadcast::channel::<(Environments, orders::Model)>(64)
    }

    pub async fn get_senders() -> Senders {
        Senders::get_senders_core().await
    }
}
