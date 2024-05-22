use std::string::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Database Error has occured")]
    Surreal(#[from] surrealdb::Error),

    #[error("Networking Error has occured")]
    Reqwest(#[from] reqwest::Error),

    #[error("Parsing Error has occured")]
    String(#[from] ParseError),

    #[error("Parsing Json Error has occured")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Tokio Tungstenite Error has occured")]
    TokioTungstenite(#[from] tokio_tungstenite::tungstenite::Error)
}