use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub id: Thing,
    pub address: String,
    chain_id: u64,
    is_executable: bool,
    native_balance: String,
}