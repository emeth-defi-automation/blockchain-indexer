use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize)]
struct DateToBlockParams {
    chain: String,
    date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct DateToBlockResponse {
    block: u64,
}

pub async fn get_block_request(
    chain: &String,
    date: DateTime<Utc>,
    moralis_api_key: &str,
    base_url: Url,
) -> Result<u64, reqwest::Error> {
    // let url = "https://deep-index.moralis.io/api/v2.2/dateToBlock";
    let url = base_url
        .join("v2.2/dateToBlock")
        .expect("Failed to join base url with stream id and address");

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
