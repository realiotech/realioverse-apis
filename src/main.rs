#![crate_name = "realioverse_api"]

use realioverse_api::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
