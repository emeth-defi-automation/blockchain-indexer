use surrealdb::{engine::remote::ws::Ws, Surreal};

use crate::models::responses::token_prices::TokenPriceResponse;

pub async fn create_token_price_history(records: Vec<TokenPriceResponse>) -> Result<(), surrealdb::Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.use_ns("test").use_db("test").await?;
    db.insert::<Vec<TokenPriceResponse>>("token_price_history").content(records).await?;
    Ok(())
}