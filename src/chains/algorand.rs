use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

/// This struct holds the Balances of each Algorand Asset
#[derive(Deserialize, Clone)]
#[allow(dead_code)]
struct Balance {
    #[allow(clippy::all)]
    address: String,
    #[allow(clippy::all)]
    amount: i64,
    #[allow(clippy::all)]
    deleted: bool,
    #[allow(clippy::all)]
    #[serde(alias = "is-frozen")]
    is_frozen: bool,
    #[allow(clippy::all)]
    #[serde(rename = "opted-in-at-round")]
    opted_in_at_round: i64,
}

/// This struct holds data required to store Algorand asset data
#[derive(Deserialize, Clone)]
#[allow(dead_code)]
pub struct Algoholder {
    balances: Vec<Balance>,
    #[serde(rename = "current-round")]
    current_round: i64,
    #[serde(rename = "next-token")]
    next_token: Option<String>,
}

/// This struct stores data about the supply of an Algorand Asset
#[derive(Debug, Serialize, Deserialize)]
pub struct AlgoSupply {
    pub asset: Asset,
    #[serde(rename = "current-round")]
    pub current_round: i64,
}

// This struct stores information about the Asset in for AlgoSupply
#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "created-at-round")]
    pub created_at_round: i64,
    pub deleted: bool,
    pub index: i64,
    pub params: Params,
}

/// This struct stores parameters about the Asset for AlgoSupply
#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub clawback: String,
    pub creator: String,
    pub decimals: i64,
    #[serde(rename = "default-frozen")]
    pub default_frozen: bool,
    pub freeze: String,
    pub manager: String,
    pub name: String,
    #[serde(rename = "name-b64")]
    pub name_b64: String,
    pub reserve: String,
    pub total: i64,
    #[serde(rename = "unit-name")]
    pub unit_name: String,
    #[serde(rename = "unit-name-b64")]
    pub unit_name_b64: String,
    pub url: String,
    #[serde(rename = "url-b64")]
    pub url_b64: String,
}

// TODO: Response is paginated.
//  Handle like this: https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html#consume-a-paginated-restful-api
//  or : http://xion.io/post/code/rust-unfold-pagination.html

/// This function returns the number of algorand holders
///
/// The function returns an `Algoholder` vector, and we get the total number of holders
/// by `Algoholder.balances.len()`
async fn get_algorand_holders() -> i64 {
    let client = reqwest::Client::new();
    let mut response;
    let mut current =
        "http://mainnet-idx.algonode.network/v2/assets/2751733/balances?currency-greater-than=0"
            .to_string();
    let mut holders: Vec<Algoholder> = Vec::new();

    loop {
        response = client
            .get(&current)
            .header("x-api-key", &env::var("ALGOD_TOKEN").unwrap())
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .header("pragma", "public")
            .send()
            .await
            .unwrap()
            .json::<Algoholder>()
            .await
            .unwrap();
        holders.push(response.clone());
        if let Some(next) = &response.next_token {
            current = format!("{}&next={}", "http://mainnet-idx.algonode.network/v2/assets/2751733/balances?currency-greater-than=0", next);
        } else {
            break;
        }
    }

    let mut sum = 0;
    for entry in &mut holders {
        sum += entry.balances.len();
    }
    sum as i64
}

/// This function returns the total supply of RIO on Algorand
async fn get_algorand_supply() -> i64 {
    let client = reqwest::Client::new();
    let response = client
        .get(&env::var("ALGOD_URL_2").unwrap())
        .header("x-api-key", &env::var("ALGOD_TOKEN").unwrap())
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("pragma", "public")
        .send()
        .await
        .unwrap()
        .json::<AlgoSupply>()
        .await
        .unwrap();

    response.asset.params.total
}

/// This function combines `get_algorand_holders and `get_algorand_supply`
pub async fn combine_algorand_data() -> (i64, i64) {
    let (holders, supply) = tokio::join!(get_algorand_holders(), get_algorand_supply());

    (holders, supply)
}
