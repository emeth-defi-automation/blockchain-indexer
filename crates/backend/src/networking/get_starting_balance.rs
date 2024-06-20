use crate::models::responses::starting_balance::GetStartingBalanceResponse;
use reqwest::Client;
use url::Url;
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
    glm_token_address: &str,
    usdc_token_address: &str,
    usdt_token_address: &str,
    moralis_api_deep_index_url: Url,
    moralis_api_key: &str,
) -> Result<Vec<GetStartingBalanceResponse>, reqwest::Error> {
    let token_addresses: Vec<&str> =
        vec![glm_token_address, usdc_token_address, usdt_token_address];
    let url = moralis_api_deep_index_url
        .join(&("v2.2/".to_owned() + wallet_address + "/erc20"))
        .unwrap();

    let client = Client::new();
    let mut request_builder = client.get(url);
    request_builder = request_builder.query(&[("chain", chain)]);
    // request_builder = request_builder.query(&[("_", &_)]);

    for (i, address) in token_addresses.iter().enumerate() {
        let param_name = format!("token_addresses[{}]", i);
        request_builder = request_builder.query(&[(param_name, address)]);
    }

    let response = request_builder
        .header("X-API-Key", moralis_api_key)
        .send()
        .await?;

    let body: Vec<GetStartingBalanceResponse> = response.json().await?;
    Ok(body)
}
