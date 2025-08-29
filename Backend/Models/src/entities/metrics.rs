use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub enum MetricType {
    Completed,
    #[default]
    All,
}
