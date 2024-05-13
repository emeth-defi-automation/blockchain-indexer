use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::post,
    Router,
};
use serde::Deserialize;
use surrealdb::engine::remote::ws::{Client, Ws};

#[derive(Clone)]
struct AppState {
    db: surrealdb::Surreal<Client>,
}

pub async fn start() {
    let app_state = AppState {
        db: surrealdb::Surreal::new::<Ws>("localhost:8000")
            .await
            .unwrap(),
    };

    let app: Router = Router::new()
        .route("/", post(handle_post))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StreamRequestBody {
    abi: serde_json::Value,
    block: Block,
    txs: serde_json::Value,
    txs_internal: serde_json::Value,
    logs: serde_json::Value,
    chain_id: String,
    confirmed: bool,
    retries: u64,
    tag: String,
    stream_id: String,
    erc20_approvals: serde_json::Value,
    erc20_transfers: Vec<Erc20Transfer>,
    nft_token_approvals: serde_json::Value,
    nft_approvals: serde_json::Value,
    nft_transfers: serde_json::Value,
    native_balances: serde_json::Value,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Erc20Transfer {
    transaction_hash: String,
    log_index: String,
    contract: String,
    #[serde(rename = "triggered_by")]
    triggered_by: Vec<String>,
    from: String,
    to: String,
    value: String,
    token_name: String,
    token_symbol: String,
    token_decimals: String,
    value_with_decimals: String,
    possible_spam: bool,
    triggers: Vec<Trigger>,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Trigger {
    name: String,
    value: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Block {
    number: String,
    hash: String,
    timestamp: String,
}

async fn handle_post(
    State(appState): State<AppState>,
    Json(body): Json<StreamRequestBody>,
) -> StatusCode {
    dbg!(&body);
    let emeth_contract_address = std::env!("EMETH_CONTRACT_ADDRESS");
    let transfers = body.erc20_transfers;
    let (number, timestamp) = (body.block.number, body.block.timestamp);
    if transfers.is_empty() || !body.confirmed {
        return StatusCode::OK;
    }
    for transfer in transfers {
        let (from, to, token_symbol, triggers) = (
            transfer.from,
            transfer.to,
            transfer.token_symbol,
            transfer.triggers,
        );
        for trigger in triggers.iter().filter(|trigger| trigger.value != "0") {
            // we need
            // timestamp, block_number, value, wallet_id, token_symbol
        }
    }

    StatusCode::OK
}
