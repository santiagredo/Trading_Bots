use std::marker::PhantomData;

use tokio::task::AbortHandle;

use crate::{
    handler::Senders,
    utils::{Core, Types},
};

#[derive(Debug, Clone)]
pub struct WebsocketStreams<Phase = Types> {
    phase: PhantomData<Phase>,
}

impl<Phase> WebsocketStreams<Phase> {
    pub fn next_phase<Next>(self) -> WebsocketStreams<Next> {
        WebsocketStreams {
            phase: PhantomData::<Next>,
        }
    }
}

impl WebsocketStreams {
    pub fn new() -> Self {
        Self {
            phase: PhantomData::<Types>,
        }
    }

    pub async fn set_binance_ws_abort_handle(abort_handle: AbortHandle) {
        WebsocketStreams::<Core>::set_binance_ws_abort_handle_core(abort_handle).await
    }

    pub async fn get_binance_ws_abort_handle() -> Option<AbortHandle> {
        WebsocketStreams::<Core>::get_binance_ws_abort_handle_core().await
    }

    pub async fn start_socket_loop(self, senders: Senders) {
        let memory_abort_handle = Self::get_binance_ws_abort_handle().await;

        if memory_abort_handle.filter(|h| !h.is_finished()).is_some() {
            return;
        }

        let abort_handle = self.next_phase().spawn_socket_loop_core(senders);
        Self::set_binance_ws_abort_handle(abort_handle).await;
    }
}
