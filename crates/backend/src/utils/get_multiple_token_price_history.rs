use crate::{
    database::{
        create_token_price_history::create_token_price_history,
        get_token_symbols::get_token_symbols,
    },
    networking::get_token_price_history::get_token_price_history,
};
use chrono::{DateTime, Duration, Utc};
use num_traits::ToPrimitive;

pub async fn get_multiple_token_price_history(date: DateTime<Utc>) -> Result<(), String> {
    let token_symbols = get_token_symbols().await.map_err(|e| e.to_string())?;
    let timestamp_iterator = 720 * 15 * 60 * 1000;
    let mut timestamp = date
        .timestamp_millis()
        .to_u64()
        .expect("Timestamp cannot be negative");
    let stop_date = date - Duration::days(368);
    let break_timestamp = stop_date
        .timestamp_millis()
        .to_u64()
        .expect("Epoch cannot be negative");
    while timestamp >= break_timestamp {
        for token in token_symbols.iter() {
            let result = get_token_price_history(token, timestamp)
                .await
                .map_err(|e| e.to_string())?;
            create_token_price_history(result)
                .await
                .map_err(|e| e.to_string())?;
            tracing::info!("Added {} historical price record", token);
        }
        timestamp = timestamp - timestamp_iterator;
    }
    Ok(())
}
