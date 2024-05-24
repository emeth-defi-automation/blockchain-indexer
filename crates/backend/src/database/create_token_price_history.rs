use crate::{models::responses::token_prices::TokenPriceResponse, DB};

pub async fn create_token_price_history(records: Vec<TokenPriceResponse>) -> Result<(), surrealdb::Error> {
    DB.insert::<Vec<TokenPriceResponse>>("token_price_history").content(records).await?;
    Ok(())
}