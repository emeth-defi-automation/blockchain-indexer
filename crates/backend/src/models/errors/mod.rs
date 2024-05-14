use std::string::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Spadłem z rowerka")]
    Surreal(#[from] surrealdb::Error),

    #[error("Ja też")]
    Reqwest(#[from] reqwest::Error),

    #[error("Ja też")]
    String(#[from] ParseError)
}