use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPriceResponse {
    pub price: String,
    pub timestamp: DateTime<Utc>,
    pub symbol: String
}