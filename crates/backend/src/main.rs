mod database;
mod models;
mod networking;
mod streams;
mod utils;

use axum_server::server::start;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use models::{errors::ServerError, wallet::Wallet};
use networking::get_block_request::get_block_request;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use streams::{
    connect_price_stream::connect_price_stream,
    handle_moralis_stream_response::handle_moralis_stream_response,
    handle_price_stream_response::handle_price_stream_response,
    handle_wallet_stream_response::handle_wallet_stream_response,
};
use surrealdb::{
    engine::remote::{ws, ws::Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};
use tokio::select;
use utils::{
    get_balance_history::get_balance_history,
    get_multiple_token_price_history::get_multiple_token_price_history,
};

static DB: Lazy<Surreal<ws::Client>> = Lazy::new(Surreal::init);

#[derive(Debug, serde::Deserialize)]
pub struct IdQueryResult {
    pub id: Thing,
}

#[derive(Debug, serde::Deserialize)]
pub struct QueryThing {
    pub tb: String,
    pub id: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CountQueryResult {
    count: i32,
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    //initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    DB.connect::<Ws>(std::env!("LOCALHOST_ADDRESS")).await?;
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    DB.use_ns(std::env!("DATABASE_NAMESPACE"))
        .use_db(std::env!("DATABASE_NAME"))
        .await?;
    let date = Utc::now();
    let chain = "sepolia".to_string();
    let to_block = get_block_request(&chain, date).await?;
    let mut current_close_time: HashMap<String, u64> = HashMap::new();
    current_close_time.insert("GLMUSDT".to_string(), 0);
    current_close_time.insert("USDCUSDT".to_string(), 0);
    let mut record_id: HashMap<String, Thing> = HashMap::new();
    let mut wallet_address_to_timestamp: HashMap<String, DateTime<Utc>> = HashMap::new();
    let (mut golem_price_stream_tx, mut golem_price_stream_rx) =
        connect_price_stream(std::env!("GLM_TOKEN_BINANCE_SYMBOL").to_lowercase())
            .await?
            .split();
    let (mut usdc_price_stream_tx, mut usdc_price_stream_rx) =
        connect_price_stream(std::env!("USDC_TOKEN_BINANCE_SYMBOL").to_lowercase())
            .await?
            .split();
    let mut wallet_balance_history_stream = DB.select::<Vec<Wallet>>("wallet").live().await?;

    tokio::spawn(async move {
        let _ = get_balance_history(to_block).await;
    });
    tokio::spawn(async move {
        let _ = get_multiple_token_price_history(date).await;
    });
    let (moralis_stream_tx, mut moralis_stream_rx) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        start(moralis_stream_tx).await;
    });

    loop {
        select! {
                Some(result) = moralis_stream_rx.recv() => {
                    handle_moralis_stream_response(result, &mut wallet_address_to_timestamp).await?;
                }
                Some(result) = wallet_balance_history_stream.next() => {
                    handle_wallet_stream_response(result, chain.clone(), &mut wallet_address_to_timestamp).await?;
                }
                Some(result) = golem_price_stream_rx.next() =>  {
                    handle_price_stream_response(result, &mut golem_price_stream_tx, &mut current_close_time, &mut record_id).await?;
                }
                Some(result) = usdc_price_stream_rx.next() => {
                    handle_price_stream_response(result, &mut usdc_price_stream_tx, &mut current_close_time, &mut record_id).await?
                }
        }
    }
}
