use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum NewOrderResponseType {
    Ack,
    ResultRes,
    Full,
}
