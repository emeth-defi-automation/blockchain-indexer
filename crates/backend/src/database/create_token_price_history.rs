use surrealdb::{engine::remote::ws::Ws, Surreal};

use crate::models::responses::token_prices::TokenPriceResponse;

pub async fn create_token_price_history(records: Vec<TokenPriceResponse>) -> Result<(), surrealdb::Error> {
    let db = Surreal::new::<Ws>(std::env!("LOCALHOST_ADDRESS")).await?;
    db.use_ns(std::env!("DATABASE_NAMESPACE")).use_db(std::env!("DATABASE_NAME")).await?;
    db.insert::<Vec<TokenPriceResponse>>("token_price_history").content(records).await?;
    Ok(())
}