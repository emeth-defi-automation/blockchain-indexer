use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct DateToBlockResponse {
    block: u64,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct DateToBlockParams {
    chain: String,
    date: DateTime<Utc>,
}

pub async fn get_block_for_date(
    chain: &String,
    date: DateTime<Utc>,
) -> Result<u64, reqwest::Error> {
    let url = "https://deep-index.moralis.io/api/v2.2/dateToBlock";
    let query = DateToBlockParams {
        chain: chain.to_string(),
        date: date,
    };
    let response = Client::new()
        .get(url)
        .query(&query)
        .header("X-API-Key", std::env!("API_KEY"))
        .send()
        .await?;
    let body: DateToBlockResponse = response.json().await?;
    Ok(body.block)
}