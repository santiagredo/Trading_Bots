pub struct Engines<R> {
    pub repo: R,
}

impl<R> Engines<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Engines<()> {
    pub fn blank() -> Engines<()> {
        Self { repo: () }
    }
}
