pub async fn add_wallet_address_to_moralis_stream(address: &str) -> Result<(), reqwest::Error> {
    let moralis_api_key = std::env!("MORALIS_API_KEY");
    let moralis_stream_id = std::env!("MORALIS_STREAM_ID");
    let moralis_api_stream_url = std::env!("MORALIS_API_STREAM_URL");
    tracing::info!(
        "Adding wallet address to Moralis stream: {}",
        moralis_stream_id
    );
    let client = reqwest::Client::new();

    let _res = client
        .post(&format!(
            "https://api.moralis-streams.com/streams/evm/{}/address",
            moralis_stream_id
        ))
        .header("accept", "application/json")
        .header("X-API-Key", moralis_api_key)
        .header("content-type", "application/json")
        .body(format!("{{\"address\": \"{}\"}}", address))
        .send()
        .await?;
    tracing::info!("{:?}", _res.text().await?);
    Ok(())
}
