#![crate_name = "realioverse_api"]

use env_logger::Env;
use realioverse_api::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to bind port");
    run(listener)?.await
}
