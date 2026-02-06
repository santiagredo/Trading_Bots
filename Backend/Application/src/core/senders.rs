use crate::handler::Senders;

impl Senders {
    pub async fn get_senders_core() -> Senders {
        Senders::get_senders_cache().await
    }
}
