use crate::utils::core::evm_exec::StateReconstructor;
use crate::utils::core::networks::Networks;
use crate::utils::core::rpc::get_block_txs_receipts;
use ethers::types::H256;

pub mod utils;

#[tokio::main]
async fn main() {
    let network = Networks::phala();
    let provider = network.rpc_provider;

    // initialize StateReconstructor with genesis.json config
    let mut reconstructor = StateReconstructor::from_genesis(&network.genesis_file);
    // reconstructor.initialize_from_genesis(network.genesis_file);

    for (address, state) in &reconstructor.accounts {
        println!("Address: {:?}, State: {:?}", address, state);
    }

    println!("\n[*] Fetching and reconstructing blocks");

    for block_nr in 0..3000 {
        match get_block_txs_receipts(provider.clone(), block_nr).await {
            Ok((block, receipts)) => {
                // apply block to the chain state
                let _ = reconstructor.apply_block(&block, &receipts);

                for tx in &block.transactions {
                    let sender = tx.from;
                    if let Some(state) = reconstructor.get_account_state(H256::from(sender)) {
                        println!("Sender state after transaction: {:?}", state);
                    }

                    if let Some(recipient) = tx.to {
                        if let Some(state) = reconstructor.get_account_state(H256::from(recipient))
                        {
                            println!("Recipient state after transaction: {:?}", state);
                        }
                    }
                }

                println!("\n[*] Fetched & reconstructed block #{:?}", block_nr);
            }
            Err(e) => {
                println!(
                    "[!] Error fetching & reconstructing block #{}: {:?}",
                    block_nr, e
                );
            }
        }
    }

    println!(
        "\n[*] State reconstruction completed from genesis until block #{}",
        3000
    );
}
