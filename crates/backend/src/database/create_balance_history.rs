use crate::{models::transfer_history_record::TransfersHistoryRecord, DB};
use serde::Deserialize;
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn create_balance_history(
    calculated_history: &Vec<TransfersHistoryRecord>,
) -> Result<(), surrealdb::Error> {
    DB.insert::<Vec<TransfersHistoryRecord>>("wallet_balance")
        .content(calculated_history)
        .await?;
    Ok(())
}
