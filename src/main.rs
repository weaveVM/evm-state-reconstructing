use crate::utils::constants::{WVM_RPC_URL, PHALA_RPC_URL};
use crate::utils::core::rpc::get_block_txs_receipts;
use crate::utils::core::genesis_load::{Genesis, load_genesis_from_file};
use crate::utils::core::state::StateReconstructor;
use ethers::types::H256;
use ethers::providers::{Provider, Http};

pub mod utils;


#[tokio::main]
async fn main() {
    let genesis: Genesis = load_genesis_from_file("./genesis/phala_mainnet.json");
    println!("{:?}", genesis);
    let provider: Provider<Http> = Provider::<Http>::try_from(PHALA_RPC_URL).unwrap();

    // initialize StateReconstructor with genesis.json config
    let mut reconstructor = StateReconstructor::new();
    reconstructor.initialize_from_genesis(genesis);

    for (address, state) in &reconstructor.accounts {
        println!("Address: {:?}, State: {:?}", address, state);
    }

    println!("\n[*] Fetching and reconstructing blocks");

    for block_nr in 0..10 {
        match get_block_txs_receipts(provider.clone(), block_nr).await {
            Ok((block, receipts)) => {

                // apply block to the chain state
                reconstructor.apply_block(&block, &receipts);

                for tx in &block.transactions {
                    let sender = tx.from;
                    if let Some(state) = reconstructor.get_account_state(H256::from(sender)) {
                        println!("Sender state after transaction: {:?}", state);
                    }

                    if let Some(recipient) = tx.to {
                        if let Some(state) = reconstructor.get_account_state(H256::from(recipient)) {
                            println!("Recipient state after transaction: {:?}", state);
                        }
                    }
                }

                println!("\n[*] Fetched & reconstructed block #{:?}", block_nr);
            }
            Err(e) => {
                println!("[!] Error fetching & reconstructing block #{}: {:?}", block_nr, e);
            }
        }
    }

    println!("\n[*] State reconstruction completed from genesis until block #{}", 3000);
}
