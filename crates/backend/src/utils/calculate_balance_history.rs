use crate::models::responses::starting_balance::GetStartingBalanceResponse;
use crate::models::responses::transfer_history::TransfersHistoryResultResponse;
use crate::models::transfer_history_record::TransfersHistoryRecord;
use crate::models::wallet::Wallet;
use chrono::DateTime;
use chrono::Utc;
use num_bigint::BigUint;
use num_traits::ops::checked::{CheckedAdd, CheckedSub};
use std::string::ParseError;
use surrealdb::sql::Datetime;

pub async fn calculate_balance_history(
    current_balance: &mut [GetStartingBalanceResponse],
    transfer_history: &[TransfersHistoryResultResponse],
    wallet: &Wallet,
) -> Result<Vec<TransfersHistoryRecord>, ParseError> {
    let mut history_records: Vec<TransfersHistoryRecord> = Vec::new();
    for token in current_balance.iter_mut() {
        let filtered_history: Vec<&TransfersHistoryResultResponse> = transfer_history
            .iter()
            .filter(|item| item.token_symbol == token.symbol)
            .collect();
        for transfer in filtered_history.iter() {
            let timestamp_converted: DateTime<Utc> = transfer.block_timestamp.parse().unwrap();
            let history_record: TransfersHistoryRecord = TransfersHistoryRecord {
                timestamp: Datetime(timestamp_converted),
                block_number: transfer.block_number.clone(),
                wallet_value: token.balance.clone(),
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
    transfer_from_address: &str,
    wallet_address: &str,
    token_balance: &str,
    transfer_value: &str,
) -> Result<BigUint, ParseError> {
    let parsed_token_balance = token_balance.parse::<BigUint>().unwrap_or_default();
    let parsed_transfer_value = transfer_value.parse::<BigUint>().unwrap_or_default();
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
