use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetStartingBalanceResponse {
    pub symbol: String,
    pub balance: String,
}
