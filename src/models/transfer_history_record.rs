use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransfersHistoryRecord {
     pub timestamp: String,
     pub block_number: String,
     pub value: String,
     pub wallet_id: Thing,
     pub token_symbol: String,
}