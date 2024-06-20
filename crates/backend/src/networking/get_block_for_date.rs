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
    moralis_api_key: &str,
) -> Result<u64, reqwest::Error> {
    let url = "https://deep-index.moralis.io/api/v2.2/dateToBlock";
    let query = DateToBlockParams {
        chain: chain.to_string(),
        date,
    };
    let response = Client::new()
        .get(url)
        .query(&query)
        .header("X-API-Key", moralis_api_key)
        .send()
        .await?;
    let body: DateToBlockResponse = response.json().await?;
    Ok(body.block)
}
