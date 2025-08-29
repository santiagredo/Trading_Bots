use tokio::task::AbortHandle;

use crate::{
    types::{Senders, WebsocketStreams},
    utils::Core,
};

impl WebsocketStreams<Core> {
    pub fn spawn_socket_loop_core(self, senders: Senders) -> AbortHandle {
        self.next_phase().spawn_socket_loop_integration(senders)
    }
}
