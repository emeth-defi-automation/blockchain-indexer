use serde::{Deserialize, Serialize};
use serde_json::Value;

pub async fn create_moralis_stream() -> Result<(), reqwest::Error> {
    // Prepare data
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
    // TODO: CREATE args with CLAP
    let client = reqwest::Client::new();
    let moralis_api_key = std::env!("MORALIS_API_KEY");
    let webhook_url = std::env!("WEBHOOK_URL").to_string();
    let description = String::from("Listen for transfers");
    let tag = String::from("transfers");
    let chain_ids = vec![std::env!("SEPOLIA_CHAIN_ID").to_string()];
    // TODO: create enums that serialize to string
    let topic0 = vec!["Transfer(address,address,uint256)".to_string()];
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
    // TODO: URL TO CONST, JOIN TO CONNECT DATA TO URL
    let serialized_stream_data =
        serde_json::to_string(&stream_data).expect("Failed to serialize stream data");
    tracing::info!("{}", serialized_stream_data);

    let moralis_api_stream_url = std::env!("MORALIS_API_STREAM_URL");
    let _res = client
        .put(moralis_api_stream_url)
        // TODO: Create struct for headers to insert one, instead of multi .header() fn calls
        .header("accept", "application/json")
        .header("X-API-Key", moralis_api_key)
        .header("content-type", "application/json")
        .body(serialized_stream_data)
        .send()
        .await?;
    // TODO: HANDLE RESPONSE to parse only necessary part that includes error msg
    // if reponses are different use flatten enum, if have similiar fields then parse to single one e.g. message field
    let json_response: Value = _res.json().await?;
    tracing::info!("{:?}", json_response);
    Ok(())
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
    webhook_url: String,
    triggers: Vec<Trigger>,
    #[serde(rename = "getNativeBalances")]
    get_native_balances: Vec<GetNativeBalances>,
}
