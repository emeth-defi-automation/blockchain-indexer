use crate::Wallet;
use surrealdb::{engine::remote::ws::Ws, Surreal};

pub async fn get_existing_wallets() -> Result<Vec<Wallet>, surrealdb::Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.use_ns("test").use_db("test").await?;
    let wallets: Vec<Wallet> = db.select("wallet").await?;
    Ok(wallets)
}
