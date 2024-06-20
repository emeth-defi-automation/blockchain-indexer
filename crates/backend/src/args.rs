use clap::Parser;
use url::Url;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, env)]
    pub moralis_api_key: String,

    #[arg(long, env)]
    pub moralis_api_stream_url: Url,

    #[arg(long, short, env)]
    pub webhook_url: Url,

    #[arg(long, env)]
    pub stream_description: String,

    #[arg(long, env)]
    pub stream_tag: String,

    #[arg(long, env)]
    pub chain_id: String,

    #[arg(long, env)]
    pub db_address: Url,

    #[arg(long, env)]
    pub db_username: String,

    #[arg(long, env)]
    pub db_password: String,

    #[arg(long, env)]
    pub db_namespace: String,

    #[arg(long, env)]
    pub db_name: String,

    #[arg(long, env)]
    pub glm_token_binance_symbol: String,

    #[arg(long, env)]
    pub usdc_token_binance_symbol: String,
}
