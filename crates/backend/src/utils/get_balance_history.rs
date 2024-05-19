use crate::database::get_existing_wallets::get_existing_wallets;
use crate::models::errors::ServerError;
use crate::utils::get_balance_history_for_wallet::get_balance_history_for_wallet;

pub async fn get_balance_history(chain: &String, to_block: u64) -> Result<(), ServerError> {
    let wallets = get_existing_wallets().await?;
    for wallet in wallets.iter() {
        get_balance_history_for_wallet(wallet, chain, to_block).await?;
    }
    Ok(())
}
