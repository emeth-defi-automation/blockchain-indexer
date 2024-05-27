use crate::models::transfer_history_record::TransfersHistoryRecord;
use serde::Deserialize;
use surrealdb::sql::Thing;
use surrealdb::{engine::remote::ws::Ws, Surreal};

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn create_balance_history(
    calculated_history: &Vec<TransfersHistoryRecord>,
) -> Result<(), surrealdb::Error> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.use_ns("test").use_db("test").await?;
    db.insert::<Vec<TransfersHistoryRecord>>("wallet_balance")
        .content(calculated_history)
        .await?;
    Ok(())
}
