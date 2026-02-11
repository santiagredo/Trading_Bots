#[derive(Debug, Default)]
pub struct OrderStatus<R> {
    pub repo: R,
}

impl<R> OrderStatus<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl OrderStatus<()> {
    pub fn blank() -> OrderStatus<()> {
        Self { repo: () }
    }
}
