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
use chrono::{DateTime, Utc};
use futures::StreamExt;
use networking::get_block_request::get_block_request;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use surrealdb::{Action, Response};
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
                    }

                    let mut balance_history_records: Vec<TransfersHistoryRecord> = Vec::new();
                    for transfer in transfers {
                        let sql = "
                            SELECT count() FROM wallet WHERE address = type::string($wallet_from_address);
                            SELECT count() FROM wallet WHERE address = type::string($wallet_to_address);
                        ";
                        let mut is_from_and_to_in_database = db.query(sql)
                            .bind(("wallet_from_address",&transfer.from)).bind(("wallet_to_address",&transfer.to))
                            .bind(("wallet_addrresultess",&transfer.from)).await?;

                        if is_from_and_to_in_database.take::<Option<i64>>(0).unwrap().unwrap() > 0
                            && result.block.clone().timestamp.parse::<DateTime<Utc>>().unwrap() > *wallet_address_to_timestamp.get(&transfer.from).unwrap() {

                            let wallet_id_query_result = db.query("SELECT id FROM wallet WHERE address = type::string($wallet_address)")
                                .bind(("wallet_address",&transfer.from)).await?;

                            let wallet_id = parse_wallet_id(wallet_id_query_result).await;

                            balance_history_records.push(TransfersHistoryRecord {
                                block_number: result.block.clone().number,
                                timestamp: result.block.clone().timestamp,
                                value: transfer.triggers[0].value.clone(),
                                wallet_id,
                                token_symbol: transfer.token_symbol.clone(),
                            });
                        }
                        if is_from_and_to_in_database.take::<Option<i64>>(1).unwrap().unwrap() > 0
                            && result.block.clone().timestamp.parse::<DateTime<Utc>>().unwrap() > *wallet_address_to_timestamp.get(&transfer.to).unwrap() {
                            let wallet_id_query_result = db.query("SELECT id FROM wallet WHERE address = type::string($wallet_address)")
                                .bind(("wallet_address",&transfer.to)).await?;
                            let wallet_id = parse_wallet_id(wallet_id_query_result).await;
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
                },
                Some(result) = stream.next() => {
                    match result {
                        Ok(notification) if notification.action == Action::Create => {
                            tracing::debug!("Received a notification: {:?}", notification.data);
                            add_wallet_address_to_moralis_stream(&notification.data.address).await?;
                            let date = Utc::now();
                            wallet_address_to_timestamp.insert(notification.data.address.clone(), date);
                            get_block_for_date(&chain, date).await?;
                            get_balance_history_for_wallet(&notification.data, &chain, to_block).await?;
                        }
                        Ok(_) => tracing::info!("Received a notification other than Create"),
                        Err(e) => tracing::error!("Error occured in select!: {}", e),

                    }
                },
        }
    }
}

async fn parse_wallet_id(mut wallet_id_query_result: Response) -> Thing {
    let wallet_id_str = wallet_id_query_result
        .take::<Option<String>>(0)
        .unwrap()
        .unwrap();
    let split: Vec<&str> = wallet_id_str.split(':').collect();
    let tb = split[0].to_string();
    let id = Id::from(split[1].to_string());
    Thing { tb, id }
}

async fn add_wallet_address_to_moralis_stream(address: &str) -> Result<(), reqwest::Error> {
    let moralis_api_key = std::env::var("MORALIS_API_KEY").expect("MORALIS_API_KEY must be set");
    let moralis_stream_id =
        std::env::var("MORALIS_STREAM_ID").expect("MORALIS_STREAM_ID must be set");
    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "https://api.moralis-streams.com/streams/evm/{}/address",
            moralis_stream_id
        ))
        .header("accept", "application/json")
        .header("X-API-Key", &moralis_api_key)
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
