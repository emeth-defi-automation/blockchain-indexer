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
    tx: tokio::sync::mpsc::Sender<StreamRequestBody>,
}

pub async fn start(tx: tokio::sync::mpsc::Sender<StreamRequestBody>) {
    let app_state = AppState {
        db: surrealdb::Surreal::new::<Ws>("localhost:8000")
            .await
            .unwrap(),
        tx,
    };

    let app: Router = Router::new()
        .route("/", post(handle_post))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown_listener())
        .await
        .unwrap();
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamRequestBody {
    abi: serde_json::Value,
    pub block: Block,
    txs: serde_json::Value,
    txs_internal: serde_json::Value,
    logs: serde_json::Value,
    chain_id: String,
    pub confirmed: bool,
    retries: u64,
    tag: String,
    stream_id: String,
    erc20_approvals: serde_json::Value,
    pub erc20_transfers: Vec<Erc20Transfer>,
    nft_token_approvals: serde_json::Value,
    nft_approvals: serde_json::Value,
    nft_transfers: serde_json::Value,
    native_balances: serde_json::Value,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Erc20Transfer {
    transaction_hash: String,
    log_index: String,
    contract: String,
    #[serde(rename = "triggered_by")]
    triggered_by: Vec<String>,
    pub from: String,
    pub to: String,
    pub value: String,
    token_name: String,
    pub token_symbol: String,
    token_decimals: String,
    value_with_decimals: String,
    possible_spam: bool,
    pub triggers: Vec<Trigger>,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trigger {
    name: String,
    pub value: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Block {
    pub number: String,
    hash: String,
    pub timestamp: String,
}

async fn handle_post(
    State(app_state): State<AppState>,
    Json(body): Json<StreamRequestBody>,
) -> StatusCode {
    app_state.tx.send(body).await.unwrap();
    StatusCode::OK
}

pub async fn graceful_shutdown_listener() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
