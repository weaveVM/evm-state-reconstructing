use crate::utils::core::evm_wvm_types::{WvmBlock, WvmTransaction, WvmTransactionReceipt};
use crate::utils::core::networks::Networks;
use anyhow::Error;
use ethereum_types::U256;
use ethers::types::{Block, Transaction, TransactionReceipt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiverInfo {
    pub first_livesync_archived_block: Option<u64>,
    pub last_livesync_archived_block: Option<u64>,
    pub first_backfill_archived_block: Option<u64>,
    pub last_backfill_archived_block: Option<u64>,
    pub livesync_start_block: Option<u64>,
    pub total_archived_blocks: Option<u64>,
    pub blocks_behind_live_blockheight: Option<u64>,
    pub archiver_balance: Option<U256>,
    pub archiver_address: Option<String>,
    pub backfill_address: Option<String>,
    pub backfill_balance: Option<U256>,
    pub network_name: Option<String>,
    pub network_chain_id: Option<u32>,
    pub network_rpc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WvmArchiverDataBlock {
    pub block: Option<WvmBlock<WvmTransaction>>,
    pub txs_receipts: Option<Vec<WvmTransactionReceipt>>,
}

pub async fn load_network_archiver_info(network: Networks) -> Result<ArchiverInfo, Error> {
    let info_url = format!("{}/v1/info", network.wvm_archiver_url.unwrap_or_default());
    let archiver_info: ArchiverInfo = reqwest::get(info_url).await?.json().await?;
    Ok(archiver_info)
}

pub async fn get_block_from_wvm(
    wvm_archiver_url: Option<String>,
    block_nr: u64,
) -> Result<(Block<Transaction>, Vec<TransactionReceipt>), Error> {
    let info_url = format!(
        "{}/v1/block/raw/{}",
        wvm_archiver_url.unwrap_or_default(),
        block_nr
    );
    let block_info: WvmArchiverDataBlock = reqwest::get(info_url).await?.json().await?;
    let block: Block<Transaction> = block_info.block.unwrap().into();
    let txs_receipts = block_info.txs_receipts.unwrap();
    let mut receipts: Vec<TransactionReceipt> = Vec::new();

    for receipt in txs_receipts {
        receipts.push(receipt.into());
    }

    Ok((block, receipts))
}
