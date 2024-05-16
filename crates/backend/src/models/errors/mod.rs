use std::string::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Database Error has occured")]
    Surreal(#[from] surrealdb::Error),

    #[error("Networking Error has occured")]
    Reqwest(#[from] reqwest::Error),

    #[error("Parsing Error has occured")]
    String(#[from] ParseError)
}