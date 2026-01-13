use crate::{
    handler::Senders,
    utils::{Cache, Core},
};

impl Senders<Core> {
    pub async fn get_senders_core() -> Senders {
        Senders::<Cache>::get_senders_cache().await
    }
}
