use crate::{
    models::{errors::ServerError, wallet::Wallet},
    networking::get_block_for_date::get_block_for_date,
    streams::{
        add_wallet_address_to_moralis_stream::add_wallet_address_to_moralis_stream,
        delete_wallet_address_from_moralis_stream::delete_wallet_address_from_moralis_stream,
    },
    utils::get_balance_history_for_wallet::get_balance_history_for_wallet,
};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use surrealdb::{Action, Error};
use url::Url;

pub async fn handle_wallet_stream_response(
    result: Result<surrealdb::Notification<Wallet>, Error>,
    chain: String,
    wallet_address_to_timestamp: &mut HashMap<String, DateTime<Utc>>,
    stream_id: &str,
    moralis_api_key: &str,
    moralis_api_stream_url: Url,
) -> Result<(), ServerError> {
    match result {
        Ok(notification) if notification.action == Action::Create => {
            tracing::debug!("Received an add notification: {:?}", notification.data);

            let address_checksummed = match notification.data.address.parse::<ethers::types::H160>()
            {
                Ok(address) => ethers::core::utils::to_checksum(&address, None),
                Err(e) => {
                    tracing::error!("Failed to parse address: {}", e);
                    return Ok(());
                }
            };
            add_wallet_address_to_moralis_stream(
                &address_checksummed,
                stream_id,
                moralis_api_key,
                moralis_api_stream_url,
            )
            .await?;

            let date = Utc::now();
            wallet_address_to_timestamp.insert(address_checksummed, date);
            let to_block = get_block_for_date(&chain, date).await?;

            get_balance_history_for_wallet(&notification.data, &chain, to_block).await?;
        }
        Ok(notification) if notification.action == Action::Delete => {
            tracing::debug!("Received a delete notification: {:?}", notification.data);

            let address_checksummed = match notification.data.address.parse::<ethers::types::H160>()
            {
                Ok(address) => ethers::core::utils::to_checksum(&address, None),
                Err(e) => {
                    tracing::error!("Failed to parse address: {}", e);
                    return Ok(());
                }
            };

            delete_wallet_address_from_moralis_stream(
                &address_checksummed,
                stream_id,
                moralis_api_key,
                moralis_api_stream_url,
            )
            .await?;
        }
        Ok(_) => tracing::info!("Received a notification other than Create"),
        Err(e) => tracing::error!("Error occured in select!: {}", e),
    }
    Ok(())
}
