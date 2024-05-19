use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransfersHistoryRecord {
    pub timestamp: Datetime,
    pub block_number: String,
    pub wallet_value: String,
    pub wallet_id: Thing,
    pub token_symbol: String,
}
