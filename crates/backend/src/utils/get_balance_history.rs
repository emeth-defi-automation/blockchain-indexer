use url::Url;

use crate::database::get_existing_wallets::get_existing_wallets;
use crate::utils::get_balance_history_for_wallet::get_balance_history_for_wallet;

pub async fn get_balance_history(
    to_block: u64,
    glm_token_address: &str,
    usdc_token_address: &str,
    usdt_token_address: &str,
    moralis_api_deep_index_url: Url,
    moralis_api_key: &str,
) -> Result<(), String> {
    let chain = "sepolia".to_string();
    let wallets = get_existing_wallets().await.map_err(|e| e.to_string())?;
    for wallet in wallets.iter() {
        get_balance_history_for_wallet(
            wallet,
            &chain,
            to_block,
            glm_token_address,
            usdc_token_address,
            usdt_token_address,
            moralis_api_deep_index_url.clone(),
            moralis_api_key,
        )
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
