use crate::models::responses::kline_binance_response::KlineDataResponse;
use surrealdb::{
    engine::remote::ws::Ws,
    sql::{Datetime, Thing},
    Surreal,
};

pub async fn add_token_price_history_record(
    record: KlineDataResponse,
) -> Result<TokenPriceResponseId, surrealdb::Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.use_ns("test").use_db("test").await?;
    let result: Vec<TokenPriceResponseId> = db
        .create("token_price_history")
        .content(TokenPriceResponse {
            price: record.close_price,
            timestamp: Datetime(DateTime::from_timestamp_millis(record.close_time as i64).unwrap()),
            symbol: record
                .symbol
                .chars()
                .take(record.symbol.len() - 4)
                .collect(),
        })
        .await?;
    Ok(result
        .into_iter()
        .nth(0)
        .expect("There will always be one item in the vector"))
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPriceResponse {
    pub price: String,
    pub timestamp: Datetime,
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPriceResponseId {
    pub id: Thing,
    pub symbol: String,
    pub timestamp: DateTime<Utc>,
}
