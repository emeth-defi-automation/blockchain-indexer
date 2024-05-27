use std::collections::HashMap;

use crate::{
    database::{
        add_token_price_history_record::add_token_price_history_record,
        update_token_price_history_record::update_token_price_history_record,
    },
    models::{errors::ServerError, responses::kline_binance_response::KlineBinanceResponse},
};
use futures::{stream::SplitSink, SinkExt};
use surrealdb::sql::Thing;
use tokio_tungstenite::tungstenite::{Error, Message};
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};

pub async fn handle_price_stream_response(
    result: Result<Message, Error>,
    write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    current_close_time: &mut HashMap<String, u64>,
    record_id: &mut HashMap<String, Thing>
) -> Result<(), ServerError> {
    match result {
        Ok(Message::Text(text)) => {
            let kline: KlineBinanceResponse = serde_json::from_str(&text)?;
            if kline.data.close_time != *current_close_time.get(&kline.data.symbol).expect("msg"){
                let new_record = add_token_price_history_record(kline.data.clone()).await?;
                current_close_time.insert(kline.data.clone().symbol, new_record.timestamp.timestamp_millis() as u64);
                record_id.insert(kline.data.clone().symbol, new_record.id);
                tracing::info!("Add {} record", kline.data.symbol);
            } else {
                let changed_record =
                    update_token_price_history_record(record_id.get(&kline.data.symbol).unwrap().clone(), kline.data.clone())
                        .await?
                        .expect("It will return a value");
                current_close_time.insert(kline.data.clone().symbol, changed_record.timestamp.timestamp_millis() as u64);
                tracing::info!("Update {} record", kline.data.symbol);
            };
        }
        Ok(Message::Ping(ping)) => {
            write.send(Message::Pong(ping)).await?;
            tracing::info!("PingPong");
        }
        Ok(Message::Pong(_)) => {
            tracing::info!("Received Pong");
        }
        Ok(Message::Close(reason)) => {
            tracing::error!("Received Close: {:?}", reason);
        }
        Err(e) => {
            tracing::error!("Error: {}", e);
        }
        _ => {}
    }
    Ok(())
}
