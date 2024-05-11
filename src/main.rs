use futures::{SinkExt, StreamExt};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AggTradeBinanceResponse {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "a")]
    pub aggregate_trade_id: u64,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub quantity: String,

    #[serde(rename = "f")]
    pub first_trade_id: u64,

    #[serde(rename = "l")]
    pub last_trade_id: u64,

    #[serde(rename = "T")]
    pub trade_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_the_market_maker: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::DEBUG)
        // sets this to be the default, global collector for this application.
        .init();

    let url = url::Url::parse("wss://stream.binance.com/ws/btcusdt@aggTrade").unwrap();

    let (ws_stream, response) = tokio_tungstenite::connect_async(url).await.unwrap();

    tracing::debug!("{:?}", response);

    let (mut write, mut read) = ws_stream.split();

    while let Some(Ok(message)) = read.next().await {
        match message {
            tokio_tungstenite::tungstenite::Message::Text(data) => {
                let result = serde_json::from_str::<AggTradeBinanceResponse>(&data).unwrap();
                tracing::info!("{:?}", result);
            }
            tokio_tungstenite::tungstenite::Message::Ping(x) => write
                .send(tokio_tungstenite::tungstenite::Message::Pong(x))
                .await
                .unwrap(),
            _ => panic!("non supported message variant"),
        }
    }
}
