use std::marker::PhantomData;

use models::entities::record_types::Model;

use crate::utils::Core;

#[derive(Debug, Default)]
pub struct RecordTypes<Phase = Core> {
    pub phase: PhantomData<Phase>,
    pub model: Model
}
