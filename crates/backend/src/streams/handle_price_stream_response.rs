use crate::{
    database::add_token_price_history_record::add_token_price_history_record,
    models::{errors::ServerError, responses::kline_binance_response::KlineBinanceResponse},
};
use futures::{stream::SplitSink, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{Error, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub async fn handle_price_stream_response(
    result: Result<Message, Error>,
    write: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
) -> Result<(), ServerError> {
    match result {
        Ok(Message::Text(text)) => {
            let kline: KlineBinanceResponse = serde_json::from_str(&text)?;
            add_token_price_history_record(kline.data.clone()).await?;
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
