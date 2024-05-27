use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KlineBinanceResponse {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "k")]
    pub data: KlineDataResponse,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KlineDataResponse {
    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "T")]
    pub close_time: u64,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "c")]
    pub close_price: String,
}