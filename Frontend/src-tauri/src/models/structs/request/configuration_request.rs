use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigurationRequest {
    pub dev_database_url: Option<String>,
    pub prod_database_url: Option<String>,
    pub api_key: Option<String>,
    pub secret_pass: Option<String>,
}
