use reqwest::{header::HeaderMap, Client, Url};

use crate::streams::delete_wallet_address_from_moralis_stream::WalletAddress;

pub async fn add_wallet_address_to_moralis_stream(
    address: &str,
    stream_id: &str,
    moralis_api_key: &str,
    moralis_api_stream_url: Url,
) -> Result<(), reqwest::Error> {
    let client = Client::new();

    let url = moralis_api_stream_url
        .join(&(stream_id.to_owned() + "/address"))
        .expect("Failed to join base url with stream id and address");

    let mut headers = HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());
    headers.insert("X-API-Key", moralis_api_key.parse().unwrap());
    headers.insert("content-type", "application/json".parse().unwrap());

    let body = WalletAddress {
        address: address.to_string(),
    };

    let serialized_body = serde_json::to_string(&body).expect("Failed to serialize stream data");

    let response = client
        .post(url)
        .headers(headers)
        .body(serialized_body)
        .send()
        .await?;

    match response.status().is_success() {
        true => {
            tracing::info!(
                "Added wallet address from Moralis stream: {:?}",
                response.text().await?
            );
        }
        false => {
            tracing::error!(
                "Failed to delete wallet address from Moralis stream: {:?}",
                response.text().await?
            );
        }
    }
    Ok(())
}
