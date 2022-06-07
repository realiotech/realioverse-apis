use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

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

/// This function returns RIO supply and holders on Ethereum
pub async fn get_ethereum_data() -> (i64, i64) {
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

    (
        response.holders_count,
        response.price.available_supply as i64,
    )
}
