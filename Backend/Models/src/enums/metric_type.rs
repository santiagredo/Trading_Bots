use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, Eq, Hash, PartialEq, Copy)]
pub enum MetricType {
    #[default]
    All,
    Completed,
}
