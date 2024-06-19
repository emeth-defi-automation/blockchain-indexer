use crate::models::responses::starting_balance::GetStartingBalanceResponse;
use reqwest::Client;
// use serde::Serialize;

// #[derive(Debug, Serialize)]
// struct GetStartingBalanceParams {
//     chain: String,
//     _: u64,
//     #[serde(rename = "token_addresses[]")]
//     token_addresses: Vec<String>,
// }

pub async fn get_starting_balance(
    wallet_address: &String,
    chain: &String,
    _to_block: u64,
) -> Result<Vec<GetStartingBalanceResponse>, reqwest::Error> {
    let glm_token_address = std::env!("GLM_TOKEN_ADDRESS").to_string();
    let usdc_token_address = std::env!("USDC_TOKEN_ADDRESS").to_string();
    let usdt_token_address = std::env!("USDT_TOKEN_ADDRESS").to_string();
    let token_addresses: Vec<String> =
        vec![glm_token_address, usdc_token_address, usdt_token_address];

    let url = format!(
        "https://deep-index.moralis.io/api/v2.2/{}/erc20",
        wallet_address
    );

    let client = Client::new();
    let mut request_builder = client.get(&url);
    request_builder = request_builder.query(&[("chain", chain)]);
    // request_builder = request_builder.query(&[("_", &_)]);

    for (i, address) in token_addresses.iter().enumerate() {
        let param_name = format!("token_addresses[{}]", i);
        request_builder = request_builder.query(&[(param_name, address)]);
    }

    let response = request_builder
        .header("X-API-Key", std::env!("MORALIS_API_KEY"))
        .send()
        .await?;

    let body: Vec<GetStartingBalanceResponse> = response.json().await?;
    Ok(body)
}
