mod database;
mod models;
mod networking;
mod utils;

use std::collections::HashMap;

use crate::models::errors::ServerError;
use crate::models::transfer_history_record::TransfersHistoryRecord;
use crate::models::wallet::Wallet;
use crate::utils::get_balance_history::get_balance_history;
use crate::utils::get_balance_history_for_wallet::get_balance_history_for_wallet;
use axum_server::server::start;

use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use networking::get_block_request::get_block_request;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::Action;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tokio::select;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.use_ns("test").use_db("test").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    let chain = "sepolia".to_string();
    let to_block = get_block_request(&chain).await?;
    get_balance_history(&chain, to_block).await?;

    let mut wallet_address_to_timestamp: HashMap<String, DateTime<Utc>> = HashMap::new();

    let mut stream = db.select::<Vec<Wallet>>("wallet").live().await?;

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        start(tx).await;
    });

    loop {
        select! {
                Some(result) = rx.recv() => {
                    let transfers = result.erc20_transfers.clone();
                    if transfers.is_empty() || !result.confirmed {
                        tracing::info!("Transfer is either empty or not confirmed");
                        continue;
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
                        let mut is_from_and_to_in_database = db.query(sql)
                            .bind(("wallet_from_address",&from_address_checksummed))
                            .bind(("wallet_to_address",&to_address_checksummed))
                            .await?;

                        let is_from: Option<CountQueryResult> = is_from_and_to_in_database.take(0)?;
                        let is_to: Option<CountQueryResult> = is_from_and_to_in_database.take(1)?;

                        if !is_from.is_none()
                            && DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(result.block.clone().timestamp.parse::<i64>().unwrap(), 0), Utc)
                            > *wallet_address_to_timestamp.get(&from_address_checksummed.to_string()).unwrap() {
                                let mut wallet_id_query_result = db.query("SELECT id FROM wallet WHERE address = type::string($wallet_address)")
                                    .bind(("wallet_address",&from_address_checksummed)).await?;

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
                            && DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(result.block.clone().timestamp.parse::<i64>().unwrap(), 0), Utc)
                                > *wallet_address_to_timestamp.get(&to_address_checksummed.to_string()).unwrap() {

                                let mut wallet_id_query_result = db.query("SELECT id FROM wallet WHERE address = type::string($wallet_address)")
                                    .bind(("wallet_address",&to_address_checksummed)).await?;

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
                            let response = db.insert::<Vec<TransfersHistoryRecord>>("rust_balance_history")
                                .content(balance_history_records).await?;
                            for record in response {
                                tracing::debug!("Inserted Record: {:?}", record);
                            }
                        }
                    }
                    Some(result) = stream.next() => {
                        match result {
                            Ok(notification) if notification.action == Action::Create => {
                                tracing::debug!("Received an add notification: {:?}", notification.data);

                                let address_checksummed = match notification.data.address.parse::<ethers::types::H160>() {
                                    Ok(address) => ethers::core::utils::to_checksum(&address, None),
                                    Err(e) => {
                                        tracing::error!("Failed to parse address: {}", e);
                                        continue;
                                    }
                                };
                                add_wallet_address_to_moralis_stream(&address_checksummed).await?;

                                let date = Utc::now();
                                wallet_address_to_timestamp.insert(address_checksummed, date);
                                let to_block = get_block_for_date(&chain, date).await?;

                                get_balance_history_for_wallet(&notification.data, &chain, to_block).await?;
                            }
                            Ok(notification) if notification.action == Action::Delete => {
                                tracing::debug!("Received a delete notification: {:?}", notification.data);

                                let address_checksummed = match notification.data.address.parse::<ethers::types::H160>() {
                                    Ok(address) => ethers::core::utils::to_checksum(&address, None),
                                    Err(e) => {
                                        tracing::error!("Failed to parse address: {}", e);
                                        continue;
                                    }
                                };

                                delete_wallet_address_from_moralis_stream(&address_checksummed).await?;
                            }
                            Ok(_) => tracing::info!("Received a notification other than Create"),
                            Err(e) => tracing::error!("Error occured in select!: {}", e),
                        }
                    },

        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct IdQueryResult {
    pub id: Thing,
}

#[derive(Debug, serde::Deserialize)]
pub struct QueryThing {
    pub tb: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
struct CountQueryResult {
    count: i32,
}

async fn delete_wallet_address_from_moralis_stream(address: &str) -> Result<(), reqwest::Error> {
    let moralis_api_key = std::env!("MORALIS_API_KEY");
    let moralis_stream_id = std::env!("MORALIS_STREAM_ID");
    let client = reqwest::Client::new();
    let res = client
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
    println!("Response: {:?}", res);
    Ok(())
}

async fn add_wallet_address_to_moralis_stream(address: &str) -> Result<(), reqwest::Error> {
    let moralis_api_key = std::env!("MORALIS_API_KEY");
    let moralis_stream_id = std::env!("MORALIS_STREAM_ID");
    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "https://api.moralis-streams.com/streams/evm/{}/address",
            moralis_stream_id
        ))
        .header("accept", "application/json")
        .header("X-API-Key", moralis_api_key)
        .header("content-type", "application/json")
        .body(format!("{{\"address\": \"{}\"}}", address))
        .send()
        .await?;
    println!("Response: {:?}", res);
    Ok(())
}

#[derive(Debug, Serialize)]
struct DateToBlockParams {
    chain: String,
    date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct DateToBlockResponse {
    block: u64,
}

pub async fn get_block_for_date(
    chain: &String,
    date: DateTime<Utc>,
) -> Result<u64, reqwest::Error> {
    let url = "https://deep-index.moralis.io/api/v2.2/dateToBlock";
    let query = DateToBlockParams {
        chain: chain.to_string(),
        date: date,
    };
    let response = Client::new()
        .get(url)
        .query(&query)
        .header("X-API-Key", std::env!("API_KEY"))
        .send()
        .await?;
    let body: DateToBlockResponse = response.json().await?;
    Ok(body.block)
}
