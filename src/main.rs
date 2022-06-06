#![crate_name = "realioverse_api"]
extern crate derive_more;
extern crate dotenv;

use derive_more::Add;
use dotenv::dotenv;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

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

/// This struct holds data required to parse Stellar User Data
#[derive(Debug, Serialize, Deserialize)]
pub struct StellarHolders {
    pub asset: String,
    pub created: i64,
    pub supply: i64,
    pub trustlines: Trustlines,
    pub payments: i64,
    pub payments_amount: i64,
    pub trades: i64,
    pub traded_amount: i64,
    pub toml_info: TomlInfo,
    pub home_domain: String,
    pub rating: Rating,
    pub price: f64,
    pub volume: i64,
    #[serde(rename = "volume7d")]
    pub volume7_d: String,
    #[serde(rename = "price7d")]
    pub price7_d: Vec<Vec<f64>>,
}

/// This struct stores rating information for StellarHolder
#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    pub age: i64,
    pub trades: i64,
    pub payments: i64,
    pub trustlines: i64,
    #[serde(rename = "volume7d")]
    pub volume7_d: i64,
    pub interop: i64,
    pub liquidity: i64,
    pub average: f64,
}

/// This struct stores tomlinfo information for StellarHolder
#[derive(Debug, Serialize, Deserialize)]
pub struct TomlInfo {
    #[serde(rename = "orgName")]
    pub org_name: String,
    #[serde(rename = "orgLogo")]
    pub org_logo: String,
    pub name: String,
    pub image: String,
    pub decimals: i64,
}

/// This struct stores trustlines information for StellarHolder
#[derive(Debug, Serialize, Deserialize)]
pub struct Trustlines {
    pub total: i64,
    pub authorized: i64,
    pub funded: i64,
}

/// This struct stores the response of ethereum api call
#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumHolders {
    pub address: String,
    pub decimals: String,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "totalSupply")]
    pub total_supply: String,
    #[serde(rename = "transfersCount")]
    pub transfers_count: i64,
    #[serde(rename = "txsCount")]
    pub txs_count: i64,
    #[serde(rename = "lastUpdated")]
    pub last_updated: i64,
    pub owner: String,
    #[serde(rename = "issuancesCount")]
    pub issuances_count: i64,
    #[serde(rename = "holdersCount")]
    pub holders_count: i64,
    pub image: String,
    pub description: String,
    pub website: String,
    #[serde(rename = "ethTransfersCount")]
    pub eth_transfers_count: i64,
    pub price: Price,
    #[serde(rename = "countOps")]
    pub count_ops: i64,
}

/// This struct stores price information for EthereumHolder
#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    pub rate: f64,
    pub diff: f64,
    #[serde(rename = "diff7d")]
    pub diff7_d: f64,
    pub ts: i64,
    #[serde(rename = "marketCapUsd")]
    pub market_cap_usd: f64,
    #[serde(rename = "availableSupply")]
    pub available_supply: f64,
    #[serde(rename = "volume24h")]
    pub volume24_h: f64,
    #[serde(rename = "volDiff1")]
    pub vol_diff1: f64,
    #[serde(rename = "volDiff7")]
    pub vol_diff7: f64,
    #[serde(rename = "volDiff30")]
    pub vol_diff30: f64,
    #[serde(rename = "diff30d")]
    pub diff30_d: f64,
    pub bid: f64,
    pub currency: String,
}

/// This struct stores the combined holders and supply for Algorand, Ethereum and Stellar
#[derive(Add, Serialize)]
pub struct CombinedData {
    pub holders: i64,
    pub supply: i64,
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
async fn combine_algorand_data() -> CombinedData {
    let (a, b) = tokio::join!(get_algorand_holders(), get_algorand_supply());

    CombinedData {
        holders: a,
        supply: b,
    }
}

/// This function returns RIO holders and supply on Stellar
async fn get_stellar_data() -> CombinedData {
    let client = reqwest::Client::new();
    let response = client
        .get(&env::var("STELLAR_URL").unwrap())
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("pragma", "public")
        .send()
        .await
        .unwrap()
        .json::<StellarHolders>()
        .await
        .unwrap();

    CombinedData {
        holders: response.trustlines.funded,
        supply: response.supply,
    }
}

/// This function returns RIO supply and holders on Ethereum
async fn get_ethereum_data() -> CombinedData {
    let client = reqwest::Client::new();
    let response = client
        .get(&env::var("ETHPLORER_URL").unwrap())
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("pragma", "public")
        .send()
        .await
        .unwrap()
        .json::<EthereumHolders>()
        .await
        .unwrap();

    CombinedData {
        holders: response.holders_count,
        supply: (response.price.available_supply as i64),
    }
}

/// This function returns RIO supply and holder information for Algorand , Ethereum and Stellar
async fn get_total_holders() -> impl Responder {
    let (algorand_holders, stellar_holders, ethereum_holders) = tokio::join! {
        combine_algorand_data(),
        get_stellar_data(),
        get_ethereum_data()
    };

    let resp = algorand_holders + stellar_holders + ethereum_holders;
    HttpResponse::Ok().json(resp)
}

/// This function implements a health check
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/get_total_holders", web::get().to(get_total_holders))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
