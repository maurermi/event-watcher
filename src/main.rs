use ethers::{
    abi::Bytes, contract::Contract, providers::{Middleware, Provider, Ws}, types::{Address, Filter, U256}
};
use eyre::Result;
use futures::StreamExt;
use std::sync::Arc;
use tokio;

// Basic ERC20 event structure
#[derive(Debug)]
struct TokenMintedEvent {
    l2Token: String,
    recipient: String,
    amount: u64,
    messageHash: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to an Ethereum node (replace with your WebSocket endpoint)
    let provider = Provider::<Ws>::connect("ws://localhost:8545").await?;
    let provider = Arc::new(provider);

    // ERC20 Transfer event signature
    let transfer_filter = Filter::new().from_block(0u64).event("TokensMinted(address,address,uint256,bytes32)");

    // Create subscription
    let mut stream = provider.subscribe_logs(&transfer_filter).await?;

    println!("Watching for TokenMinted events...");

    // Listen for events
    while let Some(log) = stream.next().await {
        println!("New TokenMinted event detected!");
        println!("Transaction hash: {}", log.transaction_hash.unwrap());
        println!("From block: {}", log.block_number.unwrap());

        // Parse the event data (simplified example)
        if log.topics.len() >= 3 {
            let event = TokenMintedEvent {
                l2Token: format!("0x{}", hex::encode(&log.topics[1].as_bytes())),
                recipient: format!("0x{}", hex::encode(&log.topics[2].as_bytes())),
                amount: u64::from_be_bytes(log.data[0..8].try_into().unwrap()),
                messageHash: format!("0x{}", hex::encode(&log.data[32..64])),
            };
            println!("Transfer: {:?}", event);
        }
    }

    Ok(())
}
