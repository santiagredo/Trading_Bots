use crate::{
    handler::Senders,
    utils::{Cache, Core},
};

impl Senders<Core> {
    pub async fn set_active_senders_core() -> Senders {
        Senders::<Cache>::set_active_senders_cache().await
    }

    pub async fn get_active_senders_core() -> Senders {
        Senders::<Cache>::get_active_senders_cache().await
    }
}
