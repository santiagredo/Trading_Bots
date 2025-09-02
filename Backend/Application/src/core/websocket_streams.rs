use tokio::task::AbortHandle;

use crate::{
    handler::{Senders, WebsocketStreams},
    utils::{Cache, Core},
};

impl WebsocketStreams<Core> {
    pub fn spawn_socket_loop_core(self, senders: Senders) -> AbortHandle {
        self.next_phase().spawn_socket_loop_integration(senders)
    }

    pub async fn set_binance_ws_abort_handle_core(abort_handle: AbortHandle) {
        WebsocketStreams::<Cache>::set_binance_ws_abort_handle_cache(abort_handle).await
    }

    pub async fn get_binance_ws_abort_handle_core() -> Option<AbortHandle> {
        WebsocketStreams::<Cache>::get_binance_ws_abort_handle_cache().await
    }
}
