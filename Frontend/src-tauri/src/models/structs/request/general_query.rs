use serde::{Deserialize, Serialize};

use crate::{
    structs::{
        ActionRequest, AssetRequest, Environments, IndicatorRequest, LedgerRequest, OrderRequest,
        PairRequest, RecordTypeRequest, StrategyRequest, TaskRequest,
    },
    traits::IntoEnvRequest,
};

#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct GeneralQuery<T> {
    pub environment: Environments,
    #[serde(flatten)]
    pub payload: T,
}

impl<T> IntoEnvRequest<T> for GeneralQuery<T> {
    fn into_env_request(self) -> (Environments, T) {
        (self.environment, self.payload)
    }
}

pub type ActionGeneralQuery = GeneralQuery<ActionRequest>;
pub type AssetGeneralQuery = GeneralQuery<AssetRequest>;
pub type IndicatorGeneralQuery = GeneralQuery<IndicatorRequest>;
pub type StrategyGeneralQuery = GeneralQuery<StrategyRequest>;
pub type PairGeneralQuery = GeneralQuery<PairRequest>;
pub type LedgerGeneralQuery = GeneralQuery<LedgerRequest>;
pub type OrderGeneralQuery = GeneralQuery<OrderRequest>;
pub type RecordTypeGeneralQuery = GeneralQuery<RecordTypeRequest>;
pub type TaskGeneralQuery = GeneralQuery<TaskRequest>;
