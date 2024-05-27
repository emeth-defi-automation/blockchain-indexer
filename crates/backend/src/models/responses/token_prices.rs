use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPriceResponse {
    pub price: String,
    pub timestamp: Datetime,
    pub symbol: String
}