#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::prelude::*;
use dotenv::dotenv;
// use ethers::prelude::k256::ecdsa::digest::DynDigest;
// use std::env;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use std::error::Error;
use std::{env, result};


use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use ethers::{
    abi::AbiDecode,
    prelude::{Address as EthAddress, *},
    utils::keccak256,
};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, path::Path, sync::Arc, time::Duration};

#[derive(Deserialize, Debug)]
struct Balance {
    address: String,
    amount: i64,
    deleted: bool,
    #[serde(alias = "is-frozen")]
    is_frozen: bool,
    #[serde(rename = "opted-in-at-round")]
    opted_in_at_round: i64,
}

/// This struct holds data required to parse Algorand User data.
#[derive(Deserialize, Debug)]
struct Algoholder {
    balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StellarHolders {
    #[serde(rename = "asset")]
    pub asset: String,

    #[serde(rename = "created")]
    pub created: i64,

    #[serde(rename = "supply")]
    pub supply: i64,

    #[serde(rename = "trustlines")]
    pub trustlines: Trustlines,

    #[serde(rename = "payments")]
    pub payments: i64,

    #[serde(rename = "payments_amount")]
    pub payments_amount: i64,

    #[serde(rename = "trades")]
    pub trades: i64,

    #[serde(rename = "traded_amount")]
    pub traded_amount: i64,

    #[serde(rename = "toml_info")]
    pub toml_info: TomlInfo,

    #[serde(rename = "home_domain")]
    pub home_domain: String,

    #[serde(rename = "rating")]
    pub rating: Rating,

    #[serde(rename = "price")]
    pub price: f64,

    #[serde(rename = "volume")]
    pub volume: i64,

    #[serde(rename = "volume7d")]
    pub volume7_d: String,

    #[serde(rename = "price7d")]
    pub price7_d: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    #[serde(rename = "age")]
    pub age: i64,

    #[serde(rename = "trades")]
    pub trades: i64,

    #[serde(rename = "payments")]
    pub payments: i64,

    #[serde(rename = "trustlines")]
    pub trustlines: i64,

    #[serde(rename = "volume7d")]
    pub volume7_d: i64,

    #[serde(rename = "interop")]
    pub interop: i64,

    #[serde(rename = "liquidity")]
    pub liquidity: i64,

    #[serde(rename = "average")]
    pub average: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlInfo {
    #[serde(rename = "orgName")]
    pub org_name: String,

    #[serde(rename = "orgLogo")]
    pub org_logo: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "image")]
    pub image: String,

    #[serde(rename = "decimals")]
    pub decimals: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trustlines {
    #[serde(rename = "total")]
    pub total: i64,

    #[serde(rename = "authorized")]
    pub authorized: i64,

    #[serde(rename = "funded")]
    pub funded: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumHolders {
    #[serde(rename = "address")]
    pub address: String,

    #[serde(rename = "decimals")]
    pub decimals: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "totalSupply")]
    pub total_supply: String,

    #[serde(rename = "transfersCount")]
    pub transfers_count: i64,

    #[serde(rename = "lastUpdated")]
    pub last_updated: i64,

    #[serde(rename = "owner")]
    pub owner: String,

    #[serde(rename = "issuancesCount")]
    pub issuances_count: i64,

    #[serde(rename = "holdersCount")]
    pub holders_count: i64,

    #[serde(rename = "image")]
    pub image: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "website")]
    pub website: String,

    #[serde(rename = "twitter")]
    pub twitter: String,

    #[serde(rename = "ts")]
    pub ts: i64,

    #[serde(rename = "ethTransfersCount")]
    pub eth_transfers_count: i64,

    #[serde(rename = "price")]
    pub price: Price,

    #[serde(rename = "countOps")]
    pub count_ops: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    #[serde(rename = "rate")]
    pub rate: f64,

    #[serde(rename = "diff")]
    pub diff: f64,

    #[serde(rename = "diff7d")]
    pub diff7_d: f64,

    #[serde(rename = "ts")]
    pub ts: i64,

    #[serde(rename = "marketCapUsd")]
    pub market_cap_usd: f64,

    #[serde(rename = "availableSupply")]
    pub available_supply: f64,

    #[serde(rename = "volume24h")]
    pub volume24_h: f64,

    #[serde(rename = "diff30d")]
    pub diff30_d: f64,

    #[serde(rename = "volDiff1")]
    pub vol_diff1: f64,

    #[serde(rename = "volDiff7")]
    pub vol_diff7: f64,

    #[serde(rename = "volDiff30")]
    pub vol_diff30: f64,

    #[serde(rename = "bid")]
    pub bid: f64,

    #[serde(rename = "currency")]
    pub currency: String,
}

async fn get_algorand_data() -> i64 {
    let client = reqwest::Client::new();
    let response = client
        .get(&env::var("ALGOD_URL").unwrap())
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
    let number_of_algo_holders = response.balances.iter().count() as i64;
    return number_of_algo_holders;
    // println!("{:?}", number_of_algo_holders);
    // Ok(())
}

async fn get_stellar_data() -> i64 {
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
    let number_of_stellar_holders = response.trustlines.funded;
    return number_of_stellar_holders;
    // println!("{:?}", number_of_stellar_holders);
    // Ok(())
}

// TODO: Add API Key as query string variable.
async fn get_ethereum_data() -> i64 {
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
    let number_of_ethereum_holders = response.holders_count;
    return number_of_ethereum_holders;
    // println!("{:?}", number_of_ethereum_holders);
    // Ok(())
}
async fn get_eth_supply() -> Result<(), Box<dyn Error>> {
    abigen!(
        RIO,
        "./abi/RIO.json",
        event_derives(serde::Deserialize, serde::Serialize)
    );

    let ethers = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
    )?;

    let ethers = Arc::new(ethers);

    let rio_eth_address = "0xf21661d0d1d76d3ecb8e1b9f1c923dbfffae4097".parse::<EthAddress>()?;

    let rio_eth_contract = RIO::new(rio_eth_address, Arc::clone(&ethers));

    let result = rio_eth_contract.total_supply().call().await?;

    println!("RIO ETH balance {:?}", result);

    Ok(())
}

async fn get_eth_total_holders() -> Result<(), Box<dyn Error>> {
    let ethers =
        Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27")
            .await?;
    let ethers = Arc::new(ethers);
    let start = ethers
        .get_block(BlockNumber::Number(U64::from(10723634)))
        .await?
        .unwrap()
        .number
        .unwrap()
        .as_u64();
    let end = ethers
        .get_block(BlockNumber::Latest)
        .await?
        .unwrap()
        .number
        .unwrap()
        .as_u64();
    let increment: u64 = 50000;
    for x in (start..end).step_by(increment as usize) {
        let rio_transfers = Filter::new()
            .address(vec!["0xf21661d0d1d76d3ecb8e1b9f1c923dbfffae4097"
                .parse::<EthAddress>()
                .unwrap()])
            .from_block(BlockNumber::from(U64::from(x)))
            .to_block(BlockNumber::from(U64::from(x + increment)))
            .topic0(ValueOrArray::Value(H256::from(keccak256(
                "Transfer(address,address,uint256)",
            ))));
        let events = ethers.get_logs(&rio_transfers).await?;
        for f in events {
            // TODO: Iterate over events and stores each unique transaction ,
            // for each transaction:
            // subtract balance from `from`
            // add balance to `to`
            // If Balance(to|from) > 0 , holder == 1
            // else holder = 0
            println!(
                "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, amount: {:?}",
                f.block_number.unwrap(),
                f.transaction_hash.unwrap(),
                f.address,
                EthAddress::from(f.topics[1]),
                EthAddress::from(f.topics[2]),
                U256::decode(f.data).unwrap()
            );
        }
    }

    Ok(())
}
// TODO : Make these requests parallel
async fn get_total_holders() -> impl Responder {
    let (algorand_holders, stellar_holders, ethereum_holders) = tokio::join! {
        get_algorand_data(),
        get_stellar_data(),
        get_ethereum_data()
    };

    let resp = algorand_holders + stellar_holders + ethereum_holders;
    HttpResponse::Ok().json(resp)
}

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
