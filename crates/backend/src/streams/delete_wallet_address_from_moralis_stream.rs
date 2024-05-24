pub async fn delete_wallet_address_from_moralis_stream(address: &str) -> Result<(), reqwest::Error> {
    let moralis_api_key = std::env!("MORALIS_API_KEY");
    let moralis_stream_id = std::env!("MORALIS_STREAM_ID");
    let client = reqwest::Client::new();
    let _res = client
        .delete(&format!(
            "https://api.moralis-streams.com/streams/evm/{}/address",
            moralis_stream_id
        ))
        .header("accept", "application/json")
        .header("X-API-Key", moralis_api_key)
        .header("content-type", "application/json")
        .body(format!("{{\"address\": \"{}\"}}", address))
        .send()
        .await?;
    println!("Deleted wallet");
    Ok(())
}