use crate::{models::{errors::ServerError, responses::{kline_binance_response::KlineDataResponse, token_prices::TokenPriceRecord}}, DB};
use surrealdb::{opt::PatchOp, sql::Thing};


pub async fn update_token_price_history_record(id: Thing, record: KlineDataResponse) -> Result<Option<TokenPriceRecord>, ServerError> {
    let result: Option<TokenPriceRecord> = DB
        .update(("token_price_history", id))
        .patch(PatchOp::replace("/price", record.close_price))
        .await?;
    Ok(result)
}
