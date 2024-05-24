use crate::DB;

pub async fn get_token_symbols() -> Result<Vec<String>,surrealdb::Error> {
    let mut result = DB.query("SELECT VALUE symbol FROM token WHERE symbol != 'USDT';").await?;
    let token_symbols: Vec<String> = result.take(0)?;
    dbg!(&token_symbols);
    Ok(token_symbols)
}

