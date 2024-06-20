use url::Url;

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
    glm_token_address: &str,
    usdc_token_address: &str,
    usdt_token_address: &str,
    moralis_api_deep_index_url: Url,
    moralis_api_key: &str,
) -> Result<(), ServerError> {
    let mut cursor = Option::None;
    let mut starting_balance: Vec<GetStartingBalanceResponse> = get_starting_balance(
        &wallet.address,
        chain,
        to_block,
        glm_token_address,
        usdc_token_address,
        usdt_token_address,
        moralis_api_deep_index_url,
        moralis_api_key,
    )
    .await?;
    loop {
        let transfer_history: GetTransfersHistoryResponse = get_transfers_history(
            &wallet.address,
            chain,
            to_block,
            cursor,
            glm_token_address,
            usdc_token_address,
            usdt_token_address,
            moralis_api_key,
        )
        .await?;
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
