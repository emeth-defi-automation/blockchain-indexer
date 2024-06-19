use crate::database::create_balance_history::create_balance_history;
use crate::models::errors::ServerError;
use crate::models::responses::starting_balance::GetStartingBalanceResponse;
use crate::models::responses::transfer_history::GetTransfersHistoryResponse;
use crate::models::wallet::Wallet;
use crate::networking::get_starting_balance::get_starting_balance;
use crate::networking::get_transfers_history::get_transfers_history;
use crate::utils::calculate_balance_history::calculate_balance_history;

pub async fn get_balance_history_for_wallet(
    wallet: &Wallet,
    chain: &String,
    to_block: u64,
) -> Result<(), ServerError> {
    let mut cursor = Option::None;
    let mut starting_balance: Vec<GetStartingBalanceResponse> =
        get_starting_balance(&wallet.address, chain, to_block).await?;
    loop {
        let transfer_history: GetTransfersHistoryResponse =
            get_transfers_history(&wallet.address, chain, to_block, cursor).await?;
        let calculated_balance_history =
            calculate_balance_history(&mut starting_balance, &transfer_history.result, wallet)
                .await?;
        create_balance_history(&calculated_balance_history).await?;
        cursor = transfer_history.cursor;
        if cursor.is_none() {
            break;
        }
    }
    Ok(())
}
