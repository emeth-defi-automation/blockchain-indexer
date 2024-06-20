use clap::Parser;
use url::Url;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, short = 'k', env)]
    pub moralis_api_key: String,

    #[arg(long, short = 'u', env)]
    pub moralis_api_stream_url: Url,

    #[arg(long, short, env)]
    pub webhook_url: Url,

    #[arg(long, short = 'd', env)]
    pub stream_description: String,

    #[arg(long, short = 't', env)]
    pub stream_tag: String,

    #[arg(long, short = 'c', env)]
    pub chain_id: String,
}
