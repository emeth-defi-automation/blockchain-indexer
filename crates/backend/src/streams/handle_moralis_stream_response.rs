use crate::{
    models::{errors::ServerError, transfer_history_record::TransfersHistoryRecord}, CountQueryResult, IdQueryResult, DB
};
use axum_server::server::StreamRequestBody;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::HashMap;
use surrealdb::sql::Thing;

pub async fn handle_moralis_stream_response(
    result: StreamRequestBody,
    wallet_address_to_timestamp: &mut HashMap<String, DateTime<Utc>>,
) -> Result<(), ServerError> {
    let transfers = result.erc20_transfers.clone();
    if transfers.is_empty() || !result.confirmed {
        tracing::info!("Transfer is either empty or not confirmed");
        return Ok(());
    }

    let mut balance_history_records: Vec<TransfersHistoryRecord> = Vec::new();
    for transfer in transfers {
        let to_address_checksummed = match transfer.to.parse::<ethers::types::H160>() {
            Ok(address) => ethers::core::utils::to_checksum(&address, None),
            Err(e) => {
                tracing::error!("Failed to parse to address: {}", e);
                break;
            }
        };

        let from_address_checksummed = match transfer.from.parse::<ethers::types::H160>() {
            Ok(address) => ethers::core::utils::to_checksum(&address, None),
            Err(e) => {
                tracing::error!("Failed to parse from address: {}", e);
                break;
            }
        };

        let sql = "
                            SELECT count() FROM wallet WHERE address = type::string($wallet_from_address);
                            SELECT count() FROM wallet WHERE address = type::string($wallet_to_address);
                        ";
        let mut is_from_and_to_in_database = DB
            .query(sql)
            .bind(("wallet_from_address", &from_address_checksummed))
            .bind(("wallet_to_address", &to_address_checksummed))
            .await?;

        let is_from: Option<CountQueryResult> = is_from_and_to_in_database.take(0)?;
        let is_to: Option<CountQueryResult> = is_from_and_to_in_database.take(1)?;

        if !is_from.is_none()
            && DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(
                    result.block.clone().timestamp.parse::<i64>().unwrap(),
                    0,
                ),
                Utc,
            ) > *wallet_address_to_timestamp
                .get(&from_address_checksummed.to_string())
                .unwrap()
        {
            let mut wallet_id_query_result = DB
                .query("SELECT id FROM wallet WHERE address = type::string($wallet_address)")
                .bind(("wallet_address", &from_address_checksummed))
                .await?;

            let wallet_data = wallet_id_query_result.take::<Vec<IdQueryResult>>(0)?;

            let wallet_id = Thing {
                tb: wallet_data[0].id.tb.clone(),
                id: wallet_data[0].id.id.clone(),
            };
            balance_history_records.push(TransfersHistoryRecord {
                block_number: result.block.clone().number,
                timestamp: result.block.clone().timestamp,
                value: transfer.triggers[0].value.clone(),
                wallet_id,
                token_symbol: transfer.token_symbol.clone(),
            });
        }

        if !is_to.is_none()
            && DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(
                    result.block.clone().timestamp.parse::<i64>().unwrap(),
                    0,
                ),
                Utc,
            ) > *wallet_address_to_timestamp
                .get(&to_address_checksummed.to_string())
                .unwrap()
        {
            let mut wallet_id_query_result = DB
                .query("SELECT id FROM wallet WHERE address = type::string($wallet_address)")
                .bind(("wallet_address", &to_address_checksummed))
                .await?;

            let wallet_data = wallet_id_query_result.take::<Vec<IdQueryResult>>(0)?;

            let wallet_id = Thing {
                tb: wallet_data[0].id.tb.clone(),
                id: wallet_data[0].id.id.clone(),
            };

            balance_history_records.push(TransfersHistoryRecord {
                block_number: result.block.clone().number,
                timestamp: result.block.clone().timestamp,
                value: transfer.triggers[1].value.clone(),
                wallet_id,
                token_symbol: transfer.token_symbol.clone(),
            });
        }
    }

    if !balance_history_records.is_empty() {
        let response = DB
            .insert::<Vec<TransfersHistoryRecord>>("rust_balance_history")
            .content(balance_history_records)
            .await?;
        for record in response {
            tracing::debug!("Inserted Record: {:?}", record);
        }
    }
    Ok(())
}
