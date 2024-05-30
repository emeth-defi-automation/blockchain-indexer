use crate::{models::responses::kline_binance_response::KlineDataResponse, DB};
use chrono::DateTime;
use surrealdb::sql::Datetime;

pub async fn add_token_price_history_record(
    record: KlineDataResponse,
) -> Result<(), surrealdb::Error> {
    let formatted_symbol: String = record
        .symbol
        .chars()
        .take(record.symbol.len() - 4)
        .collect();
    let timestamp = Datetime(DateTime::from_timestamp_millis(record.close_time as i64).unwrap());
    let price = record.close_price;
    let id = record.close_time.to_string() + &formatted_symbol;
    DB.query(
        "INSERT INTO token_price_history (id, price, timestamp, symbol) 
        VALUES ($id, $price, $timestamp, $formatted_symbol) 
        ON DUPLICATE KEY UPDATE price = $input.price;",
    )
    .bind(("id", id))
    .bind(("price", price))
    .bind(("timestamp", timestamp))
    .bind(("formatted_symbol", formatted_symbol))
    .await?;
    Ok(())
}
