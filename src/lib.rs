extern crate derive_more;
extern crate dotenv;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use derive_more::Add;
use dotenv::dotenv;
use serde::Serialize;
use std::net::TcpListener;

mod chains;
use chains::{algorand::*, ethereum::*, stellar::*};

/// This struct stores the combined holders and supply for Algorand, Ethereum and Stellar
#[derive(Add, Serialize)]
pub struct CombinedData {
    pub holders: i64,
    pub supply: i64,
}

/// This function returns RIO supply and holder information for Algorand , Ethereum and Stellar
async fn get_total_holders() -> impl Responder {
    log::info!("Calculating combined data");
    // match tokio::join!{ combine_algorand_data(),get_stellar_data(),get_ethereum_data()}{
    //     Ok(_) => {
    //         log::info!("Combined Data has been calculated")
    //         algorand_holders, stellar_holders, ethereum_holders
    //     }
    // }
    let (algorand_holders, stellar_holgit ders, ethereum_holders) = tokio::join! {
        combine_algorand_data(),
        get_stellar_data(),
        get_ethereum_data()
    };

    let combine_algorand = CombinedData {
        holders: algorand_holders.0,
        supply: algorand_holders.1,
    };
    let combine_ethereum = CombinedData {
        holders: ethereum_holders.0,
        supply: ethereum_holders.1,
    };
    let combine_stellar = CombinedData {
        holders: stellar_holders.0,
        supply: stellar_holders.1,
    };

    let resp = combine_algorand + combine_ethereum + combine_stellar;
    HttpResponse::Ok().json(resp)
}

/// This function implements a health check
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    dotenv().ok();
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/get_total_holders", web::get().to(get_total_holders))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
