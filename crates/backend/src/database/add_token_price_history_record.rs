use chrono::DateTime;
use surrealdb::sql::Datetime;
use crate::{models::responses::kline_binance_response::KlineDataResponse, DB};
use crate::models::responses::token_prices::{TokenPriceResponse, TokenPriceRecord};

pub async fn add_token_price_history_record(record: KlineDataResponse) -> Result <TokenPriceRecord, surrealdb::Error> {
    let result: Vec<TokenPriceRecord> = DB
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
