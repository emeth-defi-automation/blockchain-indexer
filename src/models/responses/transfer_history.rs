use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TransfersHistoryResultResponse {
    pub token_symbol: String,
    pub block_timestamp: String,
    pub block_number: String,
    pub to_address: String,
    pub from_address: String,
    pub value: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct GetTransfersHistoryResponse {
    total: Option<u64>,
    page: u64,
    page_size: u64,
    pub cursor: Option<String>,
    pub result: Vec<TransfersHistoryResultResponse>,
}
