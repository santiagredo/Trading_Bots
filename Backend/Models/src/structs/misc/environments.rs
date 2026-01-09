use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Default, Serialize, Copy, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Environments {
    #[default]
    DEV,
    // STG,
    // UAT,
    PROD,
}

impl Environments {
    pub fn parse(environment: &str) -> Self {
        match environment {
            "PROD" => Environments::PROD,
            // "UAT" => Environments::UAT,
            // "STG" => Environments::STG,
            _ => Environments::DEV,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Environments::PROD => "PROD",
            // Environments::UAT => "UAT",
            // Environments::STG => "STG",
            Environments::DEV => "DEV",
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Default, Serialize, Copy, Hash)]
pub struct EnvironmentsQuery {
    pub environment: Environments,
}
