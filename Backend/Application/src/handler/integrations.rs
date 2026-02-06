pub struct Integrations<R> {
    pub repo: R,
}

impl<R> Integrations<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Integrations<()> {
    pub fn blank() -> Integrations<()> {
        Self { repo: () }
    }
}
