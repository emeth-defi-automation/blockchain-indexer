use crate::database::get_existing_wallets::get_existing_wallets;
use crate::utils::get_balance_history_for_wallet::get_balance_history_for_wallet;

pub async fn get_balance_history(to_block: u64) -> Result<(),String> {
    let chain = "sepolia".to_string();
    let wallets = get_existing_wallets().await.map_err(|e| e.to_string())?;
    for wallet in wallets.iter() {
        get_balance_history_for_wallet(wallet, &chain, to_block).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}