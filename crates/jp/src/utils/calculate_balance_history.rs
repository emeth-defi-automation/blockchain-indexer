use num_bigint::BigUint;
use num_traits::ops::checked::{CheckedAdd, CheckedSub};
use std::string::ParseError;

use crate::models::responses::starting_balance::GetStartingBalanceResponse;
use crate::models::responses::transfer_history::TransfersHistoryResultResponse;
use crate::models::transfer_history_record::TransfersHistoryRecord;
use crate::models::wallet::Wallet;

pub async fn calculate_balance_history(
    current_balance: &mut Vec<GetStartingBalanceResponse>,
    transfer_history: &Vec<TransfersHistoryResultResponse>,
    wallet: &Wallet,
) -> Result<Vec<TransfersHistoryRecord>, ParseError> {
    // dbg!(&current_balance);
    // dbg!(&transfer_history);
    let mut history_records: Vec<TransfersHistoryRecord> = Vec::new();
    for token in current_balance.iter_mut() {
        // dbg!(&token);
        let filtered_history: Vec<&TransfersHistoryResultResponse> = transfer_history
            .into_iter()
            .filter(|item| item.token_symbol == token.symbol)
            .collect();
        for transfer in filtered_history.iter() {
            let history_record: TransfersHistoryRecord = TransfersHistoryRecord {
                timestamp: transfer.block_timestamp.clone(),
                block_number: transfer.block_number.clone(),
                value: token.balance.clone(),
                wallet_id: wallet.id.clone(),
                token_symbol: transfer.token_symbol.clone(),
            };
            history_records.push(history_record);
            token.balance = calculate_new_token_balance(
                &transfer.from_address,
                &wallet.address,
                &token.balance,
                &transfer.value,
            )
            .expect("Transaction balance should never overflow under 0")
            .to_string()
        }
    }
    Ok(history_records)
}

fn calculate_new_token_balance(
    transfer_from_address: &String,
    wallet_address: &String,
    token_balance: &String,
    transfer_value: &String,
) -> Result<BigUint, ParseError> {
    let parsed_token_balance = token_balance.parse::<BigUint>().unwrap_or_default();
    let parsed_transfer_value = transfer_value.parse::<BigUint>().unwrap_or_default();
    // dbg!(&parsed_token_balance);
    // dbg!(&parsed_transfer_value);
    // dbg!(&transfer_from_address);
    // dbg!(wallet_address);
    let result = if transfer_from_address.to_lowercase() == wallet_address.to_lowercase() {
        match parsed_token_balance.checked_add(&parsed_transfer_value) {
            None => panic!("Overflow"),
            Some(add) => add,
        }
    } else {
        match parsed_token_balance.checked_sub(&parsed_transfer_value) {
            None => panic!("Underflow"),
            Some(diff) => diff,
        }
    };
    Ok(result)
}
