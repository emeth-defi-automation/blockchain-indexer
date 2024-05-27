use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct DateToBlockParams {
    chain: String,
    date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct DateToBlockResponse {
    block: u64,
}

pub async fn get_block_request(chain: &String, date: DateTime<Utc>) -> Result<u64, reqwest::Error> {
    let url = "https://deep-index.moralis.io/api/v2.2/dateToBlock";
    let query = DateToBlockParams {
        chain: chain.to_string(),
        date: date,
    };
    let response = Client::new()
        .get(url)
        .query(&query)
        .header("X-API-Key", std::env!("MORALIS_API_KEY"))
        .send()
        .await?;
    let body: DateToBlockResponse = response.json().await?;
    Ok(body.block)
}
