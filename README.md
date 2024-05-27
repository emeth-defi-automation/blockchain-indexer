# blockchain-indexer

## Setup

Install Rust:

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

Create .cargo folder with config.toml file that contains:

```
[env]
GLM_TOKEN_ADDRESS = "0x054E1324CF61fe915cca47C48625C07400F1B587"
USDC_TOKEN_ADDRESS = "0xD418937d10c9CeC9d20736b2701E506867fFD85f"
USDT_TOKEN_ADDRESS = "0x9D16475f4d36dD8FC5fE41F74c9F44c7EcCd0709"
GLM_TOKEN_ETH_ADDRESS = "0x7DD9c5Cba05E151C895FDe1CF355C9A1D5DA6429"
USDC_TOKEN_ETH_ADDRESS = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
USDT_TOKEN_ETH_ADDRESS = "0xdAC17F958D2ee523a2206206994597C13D831ec7"
EMETH_CONTRACT_ADDRESS = "0x451632B70577a9D12fC8284a5C5e65DC03d8Da1a"
GLM_TOKEN_BINANCE_SYMBOL = "GLMUSDT"
USDC_TOKEN_BINANCE_SYMBOL = "USDCUSDT"
USDT_TOKEN_SYMBOL = "USDT"
MORALIS_API_KEY = 
MORALIS_STREAM_ID = 
LOCALHOST_ADDRESS = "localhost:8000"
DATABASE_NAMESPACE = "test"
DATABASE_NAME = "test"
BINANCE_KLINES_URL = "https://api.binance.com/api/v3/klines"
BINANCE_INTERVAL = "1m"
```

Create account on [Moralis](https://moralis.io/) and create new app. You will need to get `MORALIS_API_KEY` and `MORALIS_STREAM_ID` from there.

Clone [Portfolio-Management-Platform](https://github.com/emeth-defi-automation/portfolio-management-platform) repository.

## Run

### Portfolio-Management-Platform
Firstly, you need to run database:

```bash
./scripts/database-setup.sh
```

After that, you need to provision db:

```bash
./scripts/database-provision.sh
```
### Blockchain-indexer
When database is launched you can run the caching app:

```
cargo run
```

### Portfolio-Management-Platform
Before you can run the app you must launch ngrok:

```bash
npm run ngrok
```

Last step is run the app (in dev mode):

```bash
npm run dev
```