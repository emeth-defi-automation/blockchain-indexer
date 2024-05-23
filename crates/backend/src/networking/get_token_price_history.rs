use crate::models::responses::token_prices::TokenPriceResponse;
use crate::{models::errors::ServerError, utils::handle_api_ratelimit::handle_api_ratelimit};
use chrono::DateTime;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TokenPriceParams {
    symbol: String,
    interval: String,
    start_time: u64,
    end_time: u64,
    limit: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TokenPriceRawResponse {
    open_time: u64,
    open_price: String,
    high_price: String,
    low_price: String,
    pub close_price: String,
    volume: String,
    pub close_time: u64,
    asset_volume: String,
    trades: u64,
    buy_base: String,
    buy_quote: String,
    misc: String,
}

pub async fn get_token_price_history(
    token_symbol: &String,
    end_timestamp_in_millis: u64,
) -> Result<Vec<TokenPriceResponse>, ServerError> {
    let limit: u64 = 720;
    let start_time = end_timestamp_in_millis - (limit * 60 * 1000);
    let url = std::env!("BINANCE_KLINES_URL");
    let query = TokenPriceParams {
        symbol: token_symbol.to_string().to_uppercase() + std::env!("USDT_TOKEN_SYMBOL"),
        interval: std::env!("BINANCE_INTERVAL").to_string(),
        start_time: start_time,
        end_time: end_timestamp_in_millis,
        limit: limit,
    };
    let client = Client::new();
    let response = handle_api_ratelimit(3, || async {
        client.get(url).query(&query).send().await
    })
    .await?;
    let body: Vec<TokenPriceRawResponse> = response.json().await?;
    let result: Vec<TokenPriceResponse> = body
        .into_iter()
        .map(|item| TokenPriceResponse {
            price: item.close_price,
            timestamp: Datetime(DateTime::from_timestamp_millis(item.close_time as i64).unwrap()),
            symbol: token_symbol.to_string(),
        })
        .collect();
    Ok(result)
}
