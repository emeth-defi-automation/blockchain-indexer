use rand::{thread_rng, Rng};
use reqwest::{header::HeaderMap, Client};
use serde::ser::Serializer;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{Error, Value};
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

async fn create_moralis_stream(
    moralis_api_key: &str,
    moralis_api_stream_url: Url,
    webhook_url: Url,
    description: String,
    tag: String,
    chain_ids: Vec<String>,
) -> Result<CreateMoralisStreamResult, reqwest::Error> {
    let balance_of_sender_abi = FunctionABI {
        inputs: vec![ABIInput {
            internal_type: "address".to_string(),
            name: "".to_string(),
            type_field: "address".to_string(),
        }],
        name: "balanceOf".to_string(),
        outputs: vec![ABIOutput {
            internal_type: "uint256".to_string(),
            name: "fromBalance".to_string(),
            type_field: "uint256".to_string(),
        }],
        state_mutability: "view".to_string(),
        type_field: "function".to_string(),
    };

    let balance_of_receiver_abi = FunctionABI {
        inputs: vec![ABIInput {
            internal_type: "address".to_string(),
            name: "".to_string(),
            type_field: "address".to_string(),
        }],
        name: "balanceOf".to_string(),
        outputs: vec![ABIOutput {
            internal_type: "uint256".to_string(),
            name: "toBalance".to_string(),
            type_field: "uint256".to_string(),
        }],
        state_mutability: "view".to_string(),
        type_field: "function".to_string(),
    };

    let trigger_from = Trigger {
        contract_address: "$contract".to_string(),
        function_abi: balance_of_sender_abi,
        inputs: vec!["$from".to_string()],
        type_field: "erc20transfer".to_string(),
    };

    let trigger_to = Trigger {
        contract_address: "$contract".to_string(),
        function_abi: balance_of_receiver_abi,
        inputs: vec!["$to".to_string()],
        type_field: "erc20transfer".to_string(),
    };

    let triggers = vec![trigger_from, trigger_to];

    let erc20_transfer_abi = vec![ERC20TransferABI {
        anonymous: false,
        inputs: vec![
            ABIEventInput {
                indexed: true,
                internal_type: "address".to_string(),
                name: "src".to_string(),
                type_field: "address".to_string(),
            },
            ABIEventInput {
                indexed: true,
                internal_type: "address".to_string(),
                name: "dst".to_string(),
                type_field: "address".to_string(),
            },
            ABIEventInput {
                indexed: false,
                internal_type: "uint256".to_string(),
                name: "wad".to_string(),
                type_field: "uint256".to_string(),
            },
        ],
        name: "Transfer".to_string(),
        type_field: "event".to_string(),
    }];

    let topic0 = vec![serde_json::to_string(&Topic::Transfer)
        .expect("Failed to serialize Topic::Transfer")
        .trim_matches('"')
        .to_string()];

    let get_native_balances = vec![GetNativeBalances {
        selectors: vec!["$fromAddress".to_string(), "$toAddress".to_string()],
        type_field: "tx".to_string(),
    }];

    let stream_data = StreamData {
        chain_ids,
        description,
        tag,
        include_native_txs: true,
        include_contract_logs: true,
        topic0,
        abi: erc20_transfer_abi.clone(),
        webhook_url,
        triggers,
        get_native_balances,
    };

    let serialized_stream_data =
        serde_json::to_string(&stream_data).expect("Failed to serialize stream data");

    let mut headers = HeaderMap::new();
    headers.insert("accept", "application/json".parse().unwrap());
    headers.insert("X-API-Key", moralis_api_key.parse().unwrap());
    headers.insert("content-type", "application/json".parse().unwrap());

    let response = Client::new()
        .put(moralis_api_stream_url)
        .headers(headers)
        .body(serialized_stream_data)
        .send()
        .await?;

    let response_text = response.text().await?;
    match check_id_exists(&response_text) {
        Ok(Some(id)) => Ok(CreateMoralisStreamResult::Success(id)),
        Ok(None) => Ok(CreateMoralisStreamResult::Failure(response_text)),
        Err(e) => Ok(CreateMoralisStreamResult::Failure(e.to_string())),
    }
}

fn check_id_exists(data: &str) -> Result<Option<String>, Error> {
    let v: Value = serde_json::from_str(data)?;
    Ok(v.get("id").and_then(Value::as_str).map(|s| s.to_string()))
}
pub async fn create_moralis_stream_with_retries(
    max_retries: i32,
    moralis_api_key: &str,
    moralis_api_stream_url: Url,
    webhook_url: Url,
    description: String,
    tag: String,
    chain_ids: Vec<String>,
) -> Result<CreateMoralisStreamResult, reqwest::Error> {
    let mut attempt = 0;
    let mut failure_message = String::new();
    while attempt < max_retries {
        match create_moralis_stream(
            moralis_api_key,
            moralis_api_stream_url.clone(),
            webhook_url.clone(),
            description.clone(),
            tag.clone(),
            chain_ids.clone(),
        )
        .await
        {
            Ok(CreateMoralisStreamResult::Success(message)) => {
                return Ok(CreateMoralisStreamResult::Success(message));
            }
            Ok(CreateMoralisStreamResult::Failure(message)) => {
                failure_message = message;
                tracing::info!(
                    "Moralis Stream Creation Attempt {} failed, retrying...",
                    attempt + 1
                );
            }
            Err(e) => return Err(e),
        }

        attempt += 1;
        let sleep_duration = thread_rng().gen_range(3..=5);
        sleep(Duration::from_secs(sleep_duration)).await;
    }

    Ok(CreateMoralisStreamResult::Failure(failure_message))
}

#[derive(Serialize, Deserialize, Clone)]
struct ABIInput {
    #[serde(rename = "internalType")]
    internal_type: String,
    name: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct ABIOutput {
    #[serde(rename = "internalType")]
    internal_type: String,
    name: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct FunctionABI {
    inputs: Vec<ABIInput>,
    name: String,
    outputs: Vec<ABIOutput>,
    #[serde(rename = "stateMutability")]
    state_mutability: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Trigger {
    #[serde(rename = "contractAddress")]
    contract_address: String,
    #[serde(rename = "functionAbi")]
    function_abi: FunctionABI,
    inputs: Vec<String>,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct ABIEventInput {
    indexed: bool,
    #[serde(rename = "internalType")]
    internal_type: String,
    name: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct ERC20TransferABI {
    anonymous: bool,
    inputs: Vec<ABIEventInput>,
    name: String,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct GetNativeBalances {
    selectors: Vec<String>,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct StreamData {
    #[serde(rename = "chainIds")]
    chain_ids: Vec<String>,
    description: String,
    tag: String,
    #[serde(rename = "includeNativeTxs")]
    include_native_txs: bool,
    abi: Vec<ERC20TransferABI>,
    #[serde(rename = "includeContractLogs")]
    include_contract_logs: bool,
    topic0: Vec<String>,
    #[serde(rename = "webhookUrl")]
    webhook_url: Url,
    triggers: Vec<Trigger>,
    #[serde(rename = "getNativeBalances")]
    get_native_balances: Vec<GetNativeBalances>,
}

#[derive(Debug)]
enum Topic {
    Transfer,
}

impl Serialize for Topic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let topic_str = match *self {
            Topic::Transfer => "Transfer(address,address,uint256)",
        };
        serializer.serialize_str(topic_str)
    }
}

pub enum CreateMoralisStreamResult {
    Success(String),
    Failure(String),
}
