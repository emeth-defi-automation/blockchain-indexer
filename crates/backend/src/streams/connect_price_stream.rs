use reqwest::Url;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Error};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub async fn connect_price_stream(
    symbol: String,
) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Error> {
    let url = format!(
        "wss://stream.binance.com:9443/ws/{}@kline_{}",
        symbol,
        std::env!("BINANCE_INTERVAL")
    );
    let parsed_url = Url::parse(&url).unwrap();
    let (ws_stream, _) = connect_async(parsed_url).await?;
    tracing::info!("Connected to {} stream", symbol);
    Ok(ws_stream)
}
