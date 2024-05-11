use reqwest::Client;
use serde::Serialize;
use crate::models::responses::transfer_history::GetTransfersHistoryResponse;

#[derive(Debug, Serialize)]
struct GetTransfersHistoryParams {
    chain: String,
    order: String,
    limit: u64,
    to_block: u64,
    cursor: Option<String>,
}

pub async fn get_transfers_history(
    wallet_address: &String,
    chain: &String,
    to_block: u64,
    cursor: Option<String>
) -> Result<GetTransfersHistoryResponse, reqwest::Error> {
    let url = format!("https://deep-index.moralis.io/api/v2.2/{wallet_address}/erc20/transfers");
    let query = GetTransfersHistoryParams {
        chain: chain.to_string(),
        order: "DESC".to_string(),
        limit: 200,
        to_block: to_block,
        cursor: cursor,
    };
    let response = Client::new()
        .get(&url)
        .query(&query)
        .header("X-API-Key", std::env!("API_KEY"))
        .send()
        .await?;
    let body: GetTransfersHistoryResponse = response.json().await?;
    Ok(body)
}
