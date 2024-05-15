use crate::models::responses::transfer_history::GetTransfersHistoryResponse;
use reqwest::Client;
use serde::Serialize;

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
    cursor: Option<String>,
) -> Result<GetTransfersHistoryResponse, reqwest::Error> {
    let glm_token_address = std::env!("GLM_TOKEN_ADDRESS");
    let usdc_token_address = std::env!("USDC_TOKEN_ADDRESS");
    let usdt_token_address = std::env!("USDT_TOKEN_ADDRESS");
    let token_addresses: Vec<&str> =
        vec![glm_token_address, usdc_token_address, usdt_token_address];

    let url = format!(
        "https://deep-index.moralis.io/api/v2.2/{}/erc20/transfers",
        wallet_address
    );

    let client = Client::new();
    let mut request_builder = client.get(&url);
    request_builder = request_builder.query(&[("chain", chain)]);
    request_builder = request_builder.query(&[("order", &"DESC".to_string())]);
    request_builder = request_builder.query(&[("limit", &200.to_string())]);
    request_builder = request_builder.query(&[("to_block", &to_block.to_string())]);

    if let Some(cursor_value) = cursor {
        request_builder = request_builder.query(&[("cursor", &cursor_value)]);
    }

    for (i, address) in token_addresses.iter().enumerate() {
        let param_name = format!("contract_addresses[{}]", i);
        request_builder = request_builder.query(&[(param_name, address)]);
    }

    let response = request_builder
        .header("X-API-Key", std::env!("API_KEY"))
        .send()
        .await?;

    let body: GetTransfersHistoryResponse = response.json().await?;
    Ok(body)
}
