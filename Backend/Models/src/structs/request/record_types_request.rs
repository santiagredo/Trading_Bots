use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RecordTypeRequest {
    pub id: Option<i32>,
    pub name: Option<String>,
}
