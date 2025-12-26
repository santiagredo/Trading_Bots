use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Configuration {
    pub dev_database_url: String,
    pub prod_database_url: String,
    pub api_key: String,
    pub secret_pass: String,
}
