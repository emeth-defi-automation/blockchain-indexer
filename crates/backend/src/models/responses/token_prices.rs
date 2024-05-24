use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPriceResponse {
    pub price: String,
    pub timestamp: Datetime,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPriceRecord {
    pub id: Thing,
    pub symbol: String,
    pub timestamp: DateTime<Utc>
}
