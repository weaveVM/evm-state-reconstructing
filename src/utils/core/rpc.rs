use crate::utils::core::evm_wvm_types::{WvmBlock, WvmTransaction, WvmTransactionReceipt};
use ethers::middleware::Middleware;
use ethers::providers::{Http, Provider};
use ethers::types::{Block, Transaction, TransactionReceipt};
use std::sync::Arc;

pub async fn get_block_txs_receipts(
    provider: Provider<Http>,
    block_number: u64,
) -> Result<(Block<Transaction>, Vec<TransactionReceipt>), Box<dyn std::error::Error>> {
    let provider = Arc::new(provider);

    // fetch block with full transactions
    let block: Option<Block<Transaction>> = provider.get_block_with_txs(block_number).await?;
    if let Some(block) = block {
        let wvm_block: WvmBlock<WvmTransaction> = WvmBlock::from(block.clone());
        let block_2: Block<Transaction> = wvm_block.into();
        println!("{:?}", block_2);
        let mut receipts = vec![];

        // fetch receipts for each transaction
        for tx in &block.transactions {
            if let Some(receipt) = provider.get_transaction_receipt(tx.hash).await? {
                let wvm_receipt: WvmTransactionReceipt = receipt.into();
                let receipt: TransactionReceipt = wvm_receipt.into();
                // println!("{:#?}", receipt);
                receipts.push(receipt);
            } else {
                println!("Receipt not found for transaction: {:?}", tx.hash);
            }
        }
        Ok((block, receipts))
    } else {
        Err(format!("Block #{} not found", block_number).into())
    }
}
