mod database;
mod models;
mod networking;
mod utils;

use crate::models::errors::ServerError;
use crate::models::wallet::Wallet;
use crate::utils::get_balance_history::get_balance_history;
use axum_server::server::start;
use networking::get_block_request::get_block_request;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    start().await;

    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    let chain = "sepolia".to_string();
    let to_block = get_block_request(&chain).await?;
    get_balance_history(&chain, to_block).await?;
    Ok(())
}
