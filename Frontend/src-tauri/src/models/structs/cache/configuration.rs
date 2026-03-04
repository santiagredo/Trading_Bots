use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Configuration {
    pub dev_database_url: String,
    pub prod_database_url: String,
}
