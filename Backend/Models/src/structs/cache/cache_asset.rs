use serde::{Deserialize, Serialize};

use crate::entities::assets::Model;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheAsset {
    // pub is_posting: bool,
    pub model: Model,
}
