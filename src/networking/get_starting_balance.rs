use reqwest::Client;
use serde::Serialize;
use crate::models::responses::starting_balance::GetStartingBalanceResponse;

#[derive(Debug, Serialize)]
struct GetStartingBalanceParams {
    chain: String,
    to_block: u64,
    //token_addresses: Vec<String>,
}

pub async fn get_starting_balance(
    wallet_address: &String,
    chain: &String,
    to_block: u64,
) -> Result<Vec<GetStartingBalanceResponse>, reqwest::Error> {

    //let token_addresses: Vec<&str> = vec![GLM_TOKEN_ADDRESS, USDC_TOKEN_ADDRESS, USDT_TOKEN_ADDRESS];
    let url = format!("https://deep-index.moralis.io/api/v2.2/{wallet_address}/erc20");
    let query = GetStartingBalanceParams {
        chain: chain.to_string(),
        to_block: to_block,
        //token_addresses: token_addresses.iter().map(|x|x.to_string()).collect(),
    };
    let query = Client::new()
        .get(url)
        .query(&query)
        .header("X-API-Key", std::env!("API_KEY"));
    //dbg!(&query);
    let response = query.send().await?;
    let body: Vec<GetStartingBalanceResponse> = response.json().await?;
    Ok(body)
}
