#[derive(Debug, Clone)]
pub struct IntegrationLogs<R> {
    pub repo: R,
}

impl<R> IntegrationLogs<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl IntegrationLogs<()> {
    pub fn blank() -> IntegrationLogs<()> {
        Self { repo: () }
    }
}
