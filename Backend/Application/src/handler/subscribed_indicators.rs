#[derive(Debug, Clone)]
pub struct SubscribedIndicators<R> {
    pub repo: R,
}

impl SubscribedIndicators<()> {
    pub fn blank() -> SubscribedIndicators<()> {
        Self { repo: () }
    }
}
