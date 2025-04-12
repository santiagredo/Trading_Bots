use std::marker::PhantomData;

use models::entities::ledgers::Model;

use crate::utils::Core;

#[derive(Debug, Default)]
pub struct Ledgers<Phase = Core> {
    pub phase: PhantomData<Phase>,
    pub model: Model,
}
