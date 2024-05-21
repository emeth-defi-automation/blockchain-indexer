use crate::models::{errors::ServerError, responses::kline_binance_response::KlineDataResponse};
use surrealdb::{engine::remote::ws::Ws, opt::PatchOp, sql::Thing, Surreal};

pub async fn update_token_price_history_record(id: Thing, record: KlineDataResponse) -> Result<Option<TokenPriceResponseId>, ServerError> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.use_ns("test").use_db("test").await?;
    let result: Option<TokenPriceResponseId> = db
        .update(("token_price_history", id))
        .patch(PatchOp::replace("/price", record.close_price)).await?;
    Ok(result)
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPriceResponse {
    id: Option<String>,
    pub price: String,
    pub timestamp: DateTime<Utc>,
    pub symbol: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPriceResponseId {
    pub id: Thing,
    pub timestamp: DateTime<Utc>
}