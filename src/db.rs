// use diesel::prelude::*;
// use dotenv::dotenv;
// use std::env;

// use models::*;
// use schema::ethbalances;
// use schema::ethbalances::dsl::*;

// fn establish_connection() -> SqliteConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     SqliteConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

// pub fn get_eth_balances() -> Vec<U256> {
//     let connection = establish_connection();
//     let results = ethbalances
//         .filter(holder.eq(true))
//         .limit(5)
//         .load::<GetETHRioBalance>(&connection)
//         .expect("Error loading Balances");

//     println!("Displaying {} balances", results.len());
//     for balances in results {
//         println!("{:?}", balances.account);
//         println!("----------\n");
//         println!("{:?}", balances.balance);
//         println!("----------\n");
//         println!("{:?}", balances.holder);
//     }

//     return results;
// }

// pub fn post_eth_balances() -> EthBalance {
//     let ethers =
//         Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27")
//             .await?;
//     let ethers = Arc::new(ethers);
//     let start = ethers
//         .get_block(BlockNumber::Number(U64::from(10723634)))
//         .await?
//         .unwrap()
//         .number
//         .unwrap()
//         .as_u64();
//     let end = ethers
//         .get_block(BlockNumber::Latest)
//         .await?
//         .unwrap()
//         .number
//         .unwrap()
//         .as_u64();
//     let increment: u64 = 50000;
//     for x in (start..end).step_by(increment as usize) {
//         let rio_transfers = Filter::new()
//             .address(vec!["0xf21661d0d1d76d3ecb8e1b9f1c923dbfffae4097"
//                 .parse::<EthAddress>()
//                 .unwrap()])
//             .from_block(BlockNumber::from(U64::from(x)))
//             .to_block(BlockNumber::from(U64::from(x + increment)))
//             .topic0(ValueOrArray::Value(H256::from(keccak256(
//                 "Transfer(address,address,uint256)",
//             ))));
//         let events = ethers.get_logs(&rio_transfers).await?;
//         for f in events {
//             // TODO: Iterate over events and stores each unique transaction ,
//             // for each transaction:
//             // subtract balance from `from`
//             // add balance to `to`
//             // If Balance(to|from) > 0 , holder == 1
//             // else holder = 0
//             println!(
//                 "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, amount: {:?}",
//                 f.block_number.unwrap(),
//                 f.transaction_hash.unwrap(),
//                 f.address,
//                 EthAddress::from(f.topics[1]),
//                 EthAddress::from(f.topics[2]),
//                 U256::decode(f.data).unwrap()
//             );
//         }
//     }

//     Ok(())
// }
