use crate::{
    database::{
        add_token_price_history_record::add_token_price_history_record,
        update_token_price_history_record::update_token_price_history_record,
    },
    models::{errors::ServerError, responses::kline_binance_response::KlineBinanceResponse},
};
use futures::{SinkExt, StreamExt};
use reqwest::Url;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub async fn get_price_stream(symbol: String) -> Result<(), ServerError> {
    let url = format!("wss://stream.binance.com/ws/{symbol}@kline_1m");
    let parsed_url = Url::parse(&url).unwrap();
    let (ws_stream, _) = connect_async(parsed_url).await?;
    let (mut write, mut read) = ws_stream.split();
    let mut start_time: u64 = 0;
    let mut id = Option::None;
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                let kline: KlineBinanceResponse = serde_json::from_str(&text)?;
                if kline.data.close_time != start_time {
                    let new_record = add_token_price_history_record(kline.data).await?;
                    start_time = new_record.timestamp.timestamp_millis() as u64;
                    id = Some(new_record.id);
                } else {
                    let changed_record = update_token_price_history_record(
                        id.clone().unwrap(),
                        kline.data,
                    )
                    .await?
                    .expect("It will return a value");
                    start_time = changed_record.timestamp.timestamp_millis() as u64;
                };
            }
            Ok(Message::Ping(ping)) => {
                write.send(Message::Pong(ping)).await?;
            }
            Ok(Message::Pong(_)) => {
                tracing::info!("Received Pong for symbol: {}", symbol);
            }
            Ok(Message::Close(reason)) => {
                tracing::error!("Received Close for symbol {}: {:?}", symbol, reason);
                break;
            }
            Err(e) => {
                tracing::error!("Error for symbol {}: {}", symbol, e);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
