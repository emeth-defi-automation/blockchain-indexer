use crate::{Wallet, DB};

pub async fn get_existing_wallets() -> Result<Vec<Wallet>, surrealdb::Error> {
    let wallets: Vec<Wallet> = DB.select("wallet").await?;
    Ok(wallets)
}
