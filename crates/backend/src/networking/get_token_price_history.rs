use crate::models::responses::token_prices::TokenPriceResponse;
use crate::{models::errors::ServerError, utils::handle_api_ratelimit::handle_api_ratelimit};
use chrono::DateTime;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
use url::Url;

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
    binance_klines_url: Url,
    binance_interval: &str,
) -> Result<Vec<TokenPriceResponse>, ServerError> {
    let limit: u64 = 720;
    let start_time = end_timestamp_in_millis - (limit * 60 * 1000);

    let query = TokenPriceParams {
        symbol: token_symbol.to_string().to_uppercase() + "USDT",
        interval: binance_interval.to_string(),
        start_time,
        end_time: end_timestamp_in_millis,
        limit,
    };
    let client = Client::new();
    let response = handle_api_ratelimit(3, || async {
        client
            .get(binance_klines_url.clone())
            .query(&query)
            .send()
            .await
    })
    .await?;
    let body: Vec<TokenPriceRawResponse> = response.json().await?;
    let result: Vec<TokenPriceResponse> = body
        .into_iter()
        .map(|item| TokenPriceResponse {
            id: Thing {
                tb: "token_price_history".to_string(),
                id: Id::String(item.close_time.to_string() + token_symbol),
            },
            price: item.close_price,
            timestamp: Datetime(DateTime::from_timestamp_millis(item.close_time as i64).unwrap()),
            symbol: token_symbol.to_string(),
        })
        .collect();
    Ok(result)
}
