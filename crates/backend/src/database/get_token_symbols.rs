use surrealdb::{engine::remote::ws::Ws, Surreal};

pub async fn get_token_symbols() -> Result<Vec<String>, surrealdb::Error> {
    let db = Surreal::new::<Ws>(std::env!("LOCALHOST_ADDRESS")).await?;
    db.use_ns(std::env!("DATABASE_NAMESPACE"))
        .use_db(std::env!("DATABASE_NAME"))
        .await?;
    let mut result = db
        .query("SELECT VALUE symbol FROM token WHERE symbol != 'USDT';")
        .await?;
    let token_symbols: Vec<String> = result.take(0)?;
    dbg!(&token_symbols);
    Ok(token_symbols)
}
