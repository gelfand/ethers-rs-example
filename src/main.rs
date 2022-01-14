use client::ClientMiddleware;
use ethers::prelude::{Provider, Ws};
use std::time::Duration;

pub mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let provider = ClientMiddleware::new(
        Provider::new(Ws::connect("ws://127.0.0.1:8545").await.unwrap())
            .interval(Duration::from_millis(100)),
    );

    provider.listen_transactions().await.unwrap();

    Ok(())
}
