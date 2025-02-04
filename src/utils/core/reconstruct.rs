use crate::utils::core::evm_exec::StateReconstructor;
use crate::utils::core::networks::Networks;
use crate::utils::core::wvm_archiver::{get_block_from_wvm, load_network_archiver_info};
use anyhow::Error;
use ethers::types::H256;

pub async fn reconstruct_network(network: Networks) -> Result<StateReconstructor, Error> {
    let mut reconstructor = StateReconstructor::from_genesis(&network.genesis_file);

    for (address, state) in &reconstructor.accounts {
        println!("Address: {:?}, State: {:?}", address, state);
    }

    let wvm_archiver_info = load_network_archiver_info(network.clone()).await?;
    let backfill_start_block = wvm_archiver_info
        .first_backfill_archived_block
        .unwrap_or_default();
    let backfill_end_block = wvm_archiver_info
        .last_backfill_archived_block
        .unwrap_or_default();

    println!("\n[*] Fetching and reconstructing blocks");

    for block_nr in backfill_start_block..backfill_end_block {
        match get_block_from_wvm(network.wvm_archiver_url.clone(), block_nr).await {
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

                println!(
                    "\n[*] Fetched from WeaveVM & reconstructed block #{:?}",
                    block_nr
                );
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
        "\n[*] State reconstruction completed from block #{} until block #{}",
        backfill_start_block, backfill_end_block
    );

    Ok(reconstructor)
}
