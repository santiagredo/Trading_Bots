use std::{marker::PhantomData, sync::Arc};

use once_cell::sync::Lazy;
use tokio::{sync::RwLock, task::AbortHandle};

use crate::{types::Senders, utils::Types};

#[derive(Debug, Clone)]
pub struct WebsocketStreams<Phase = Types> {
    phase: PhantomData<Phase>,
}

static BINANCE_WS_ABORT_HANDLE: Lazy<Arc<RwLock<Option<AbortHandle>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl WebsocketStreams {
    pub fn new() -> Self {
        Self {
            phase: PhantomData::<Types>,
        }
    }

    pub async fn set_binance_ws_abort_handle(abort_handle: AbortHandle) {
        let mut memory_abort_handle_lock = BINANCE_WS_ABORT_HANDLE.write().await;

        if let Some(memory_abort_handle) = memory_abort_handle_lock.as_ref() {
            memory_abort_handle.abort();
        }

        *memory_abort_handle_lock = Some(abort_handle);
    }

    pub async fn get_binance_ws_abort_handle() -> Option<AbortHandle> {
        let memory_abort_handle_lock = BINANCE_WS_ABORT_HANDLE.read().await;

        memory_abort_handle_lock.clone()
    }
}

impl<Phase> WebsocketStreams<Phase> {
    pub fn next_phase<Next>(self) -> WebsocketStreams<Next> {
        WebsocketStreams {
            phase: PhantomData::<Next>,
        }
    }
}

impl WebsocketStreams<Types> {
    pub async fn start_socket_loop(self, senders: Senders) {
        let memory_abort_handle = Self::get_binance_ws_abort_handle().await;

        if memory_abort_handle.filter(|h| !h.is_finished()).is_some() {
            return;
        }

        let abort_handle = self.next_phase().spawn_socket_loop_core(senders);
        Self::set_binance_ws_abort_handle(abort_handle).await;
    }
}
