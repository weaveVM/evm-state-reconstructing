use crate::utils::core::evm_exec::StateReconstructor;
use crate::utils::core::networks::Networks;
use crate::utils::core::reconstruct::reconstruct_network;
use anyhow::Error;

pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let network = Networks::rss3();
    let state: StateReconstructor = reconstruct_network(network).await?;
    Ok(())
}
