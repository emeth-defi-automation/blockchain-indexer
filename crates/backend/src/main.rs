mod args;
mod database;
mod models;
mod networking;
mod streams;
mod utils;

use args::Args;
use axum_server::server::{graceful_shutdown_listener, start};
use chrono::{DateTime, Utc};
use clap::Parser;
use futures::{future::join_all, StreamExt};
use models::{errors::ServerError, wallet::Wallet};
use networking::get_block_request::get_block_request;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use streams::{
    connect_price_stream::connect_price_stream,
    create_moralis_stream::{create_moralis_stream_with_retries, CreateMoralisStreamResult},
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

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

    DB.connect::<Ws>(args.db_address.to_string()).await?;

    DB.signin(Root {
        username: &args.db_username,
        password: &args.db_password,
    })
    .await?;

    DB.use_ns(args.db_namespace).use_db(args.db_name).await?;

    let date = Utc::now();
    let chain = "sepolia".to_string();
    let to_block = get_block_request(
        &chain,
        date,
        &args.moralis_api_key,
        args.moralis_api_deep_index_url.clone(),
    )
    .await?;
    let mut wallet_address_to_timestamp: HashMap<String, DateTime<Utc>> = HashMap::new();

    let (mut golem_price_stream_tx, mut golem_price_stream_rx) = connect_price_stream(
        args.glm_token_binance_symbol.to_lowercase(),
        &args.binance_interval,
    )
    .await?
    .split();
    let (mut usdc_price_stream_tx, mut usdc_price_stream_rx) = connect_price_stream(
        args.usdc_token_binance_symbol.to_lowercase(),
        &args.binance_interval,
    )
    .await?
    .split();
    let mut wallet_balance_history_stream = DB.select::<Vec<Wallet>>("wallet").live().await?;

    // Clone because of async move
    let glm_token_address = args.glm_token_address.clone();
    let usdc_token_address = args.usdc_token_address.clone();
    let usdt_token_address = args.usdt_token_address.clone();
    let moralis_api_deep_index_url = args.moralis_api_deep_index_url.clone();
    let moralis_api_key = args.moralis_api_key.clone();

    tokio::spawn(async move {
        let _ = get_balance_history(
            to_block,
            &glm_token_address,
            &usdc_token_address,
            &usdt_token_address,
            moralis_api_deep_index_url,
            &moralis_api_key,
        )
        .await;
    });

    tokio::spawn(async move {
        let _ =
            get_multiple_token_price_history(date, args.binance_klines_url, &args.binance_interval)
                .await;
    });

    let (moralis_stream_tx, mut moralis_stream_rx) = tokio::sync::mpsc::channel(100);

    let axum_task = tokio::spawn(async move {
        start(moralis_stream_tx).await;
    });

    let mut message_from_moralis_stream_creation = String::new();
    match create_moralis_stream_with_retries(
        10,
        &args.moralis_api_key,
        args.moralis_api_stream_url.clone(),
        args.webhook_url,
        args.stream_description,
        args.stream_tag,
        vec![args.chain_id],
    )
    .await
    {
        Ok(CreateMoralisStreamResult::Success(message)) => {
            message_from_moralis_stream_creation = message;
            tracing::info!(
                "Moralis stream created: {}",
                message_from_moralis_stream_creation
            );
        }
        Ok(CreateMoralisStreamResult::Failure(message)) => {
            tracing::error!("Failed to create Moralis stream: {}", message);
        }
        Err(e) => {
            tracing::error!("Failed to create Moralis stream: {}", e);
        }
    }

    let (shutdown_tx, mut shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let shutdown_task = tokio::spawn(async move {
        graceful_shutdown_listener().await;
        let _ = shutdown_tx.send(());
    });

    loop {
        select! {
                Some(result) = moralis_stream_rx.recv() => {
                    handle_moralis_stream_response(result, &mut wallet_address_to_timestamp).await?;
                }
                Some(result) = wallet_balance_history_stream.next() => {
                    handle_wallet_stream_response(result, chain.clone(), &mut wallet_address_to_timestamp, &message_from_moralis_stream_creation, &args.moralis_api_key, args.moralis_api_stream_url.clone(), &args.glm_token_address, &args.usdc_token_address, &args.usdt_token_address, args.moralis_api_deep_index_url.clone()).await?;
                }
                Some(result) = golem_price_stream_rx.next() =>  {
                    handle_price_stream_response(result, &mut golem_price_stream_tx).await?;
                }
                Some(result) = usdc_price_stream_rx.next() => {
                    handle_price_stream_response(result, &mut usdc_price_stream_tx).await?
                }
                _  = &mut shutdown_rx => {
                    println!("Shutting down");
                    break;
                }
        }
    }
    let _ = join_all([axum_task, shutdown_task]).await;
    println!("Graceful shutdown");
    Ok(())
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CountQueryResult {
    count: i32,
}
