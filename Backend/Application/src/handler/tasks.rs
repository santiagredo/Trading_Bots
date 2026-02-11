#[derive(Debug, Clone)]
pub struct Tasks<R> {
    pub repo: R,
}

impl<R> Tasks<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Tasks<()> {
    pub fn blank() -> Tasks<()> {
        Self { repo: () }
    }
}
