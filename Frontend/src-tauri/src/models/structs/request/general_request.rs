use serde::{Deserialize, Serialize};

use crate::{
    structs::{
        ActionRequest, AssetRequest, Environments, IndicatorRequest, LedgerRequest, OrderRequest,
        PairRequest, RecordTypeRequest, StrategyRequest, TaskRequest,
    },
    traits::IntoEnvRequest,
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GeneralRequest<T> {
    pub environment: Environments,
    pub payload: T,
}

impl<T> IntoEnvRequest<T> for GeneralRequest<T> {
    fn into_env_request(self) -> (Environments, T) {
        (self.environment, self.payload)
    }
}

pub type ActionGeneralRequest = GeneralRequest<ActionRequest>;
pub type AssetGeneralRequest = GeneralRequest<AssetRequest>;
pub type IndicatorGeneralRequest = GeneralRequest<IndicatorRequest>;
pub type StrategyGeneralRequest = GeneralRequest<StrategyRequest>;
pub type PairGeneralRequest = GeneralRequest<PairRequest>;
pub type LedgerGeneralRequest = GeneralRequest<LedgerRequest>;
pub type OrderGeneralRequest = GeneralRequest<OrderRequest>;
pub type RecordTypeGeneralRequest = GeneralRequest<RecordTypeRequest>;
pub type TaskGeneralRequest = GeneralRequest<TaskRequest>;
