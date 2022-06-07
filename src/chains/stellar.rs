use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

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

/// This function returns RIO holders and supply on Stellar
pub async fn get_stellar_data() -> (i64, i64) {
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
    (response.trustlines.funded, response.supply)
}
