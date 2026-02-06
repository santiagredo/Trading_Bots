#[derive(Debug, Clone)]
pub struct ErrorLogs<R> {
    pub repo: R,
}

impl<R> ErrorLogs<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl ErrorLogs<()> {
    pub fn blank() -> ErrorLogs<()> {
        Self { repo: () }
    }
}
