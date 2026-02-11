#[derive(Debug, Clone)]
pub struct Ledgers<R> {
    pub repo: R,
}

impl<R> Ledgers<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Ledgers<()> {
    pub fn blank() -> Ledgers<()> {
        Self { repo: () }
    }
}
