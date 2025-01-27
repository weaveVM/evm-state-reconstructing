use crate::utils::core::serde_arrays::{deserialize_256, serialize_256};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use ethers::types::transaction::eip2930::AccessList;
use ethers::types::transaction::eip2930::AccessListItem;
use ethers::types::{
    Address, Block, Bytes, Log, Transaction, TransactionReceipt, Withdrawal, H256, U256, U64,
};
use hex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(
    BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default,
)]
pub struct WvmTransaction {
    // Optimism fields
    pub source_hash: Option<[u8; 32]>,
    pub mint: Option<[u8; 32]>,
    pub is_system_tx: bool,

    // Celo fields
    pub fee_currency: Option<[u8; 20]>,
    pub gateway_fee_recipient: Option<[u8; 20]>,
    pub gateway_fee: Option<[u8; 32]>,

    // Base transaction fields
    pub hash: [u8; 32],
    pub nonce: [u8; 32],
    pub block_hash: Option<[u8; 32]>,
    pub block_number: Option<u64>,
    pub transaction_index: Option<u64>,
    pub from: [u8; 20],
    pub to: Option<[u8; 20]>,
    pub value: [u8; 32],
    pub gas_price: Option<[u8; 32]>,
    pub gas: [u8; 32],
    pub input: Vec<u8>,
    pub v: u64,
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub transaction_type: Option<u64>,
    pub access_list: Option<Vec<(Vec<u8>, Vec<[u8; 32]>)>>,
    pub max_priority_fee_per_gas: Option<[u8; 32]>,
    pub max_fee_per_gas: Option<[u8; 32]>,
    pub chain_id: Option<[u8; 32]>,
}

#[derive(
    BorshSerialize, BorshDeserialize, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize,
)]
pub struct WvmBlock<WvmTransaction> {
    pub hash: Option<[u8; 32]>,
    pub parent_hash: [u8; 32],
    pub uncles_hash: [u8; 32],
    pub author: Option<[u8; 20]>,
    pub state_root: [u8; 32],
    pub transactions_root: [u8; 32],
    pub receipts_root: [u8; 32],
    pub number: Option<u64>,
    pub gas_used: [u8; 32],
    pub gas_limit: [u8; 32],
    pub extra_data: Vec<u8>,
    #[serde(serialize_with = "serialize_256", deserialize_with = "deserialize_256")]
    pub logs_bloom: Option<[u8; 256]>,
    pub timestamp: [u8; 32],
    pub difficulty: [u8; 32],
    pub total_difficulty: Option<[u8; 32]>,
    pub seal_fields: Vec<Vec<u8>>,
    pub uncles: Vec<[u8; 32]>,
    pub transactions: Vec<WvmTransaction>,
    pub size: Option<[u8; 32]>,
    pub mix_hash: Option<[u8; 32]>,
    pub nonce: Option<[u8; 8]>,
    pub base_fee_per_gas: Option<[u8; 32]>,
    pub blob_gas_used: Option<[u8; 32]>,
    pub excess_blob_gas: Option<[u8; 32]>,
    pub withdrawals_root: Option<[u8; 32]>,
    pub withdrawals: Option<Vec<WvmWithdrawal>>,
    pub parent_beacon_block_root: Option<[u8; 32]>,
}

#[derive(
    BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default,
)]
pub struct WvmWithdrawal {
    pub index: u64,
    pub validator_index: u64,
    pub address: [u8; 20],
    pub amount: u64,
}

#[derive(
    BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default,
)]
pub struct WvmTransactionReceipt {
    pub transaction_hash: [u8; 32],
    pub transaction_index: u64,
    pub block_hash: Option<[u8; 32]>,
    pub block_number: Option<u64>,
    pub from: [u8; 20],
    pub to: Option<[u8; 20]>,
    pub cumulative_gas_used: [u8; 32],
    pub gas_used: Option<[u8; 32]>,
    pub contract_address: Option<[u8; 20]>,
    pub logs: Vec<WvmLog>,
    pub status: Option<u64>,
    pub root: Option<[u8; 32]>,
    #[serde(serialize_with = "serialize_256", deserialize_with = "deserialize_256")]
    pub logs_bloom: Option<[u8; 256]>,
    pub transaction_type: Option<u64>,
    pub effective_gas_price: Option<[u8; 32]>,

    // Optimism fields
    pub deposit_nonce: Option<u64>,
    pub l1_fee: Option<[u8; 32]>,
    pub l1_fee_scalar: Option<[u8; 32]>,
    pub l1_gas_price: Option<[u8; 32]>,
    pub l1_gas_used: Option<[u8; 32]>,
}

#[derive(
    BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default,
)]
pub struct WvmLog {
    pub address: [u8; 20],
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
    pub block_hash: Option<[u8; 32]>,
    pub block_number: Option<u64>,
    pub transaction_hash: Option<[u8; 32]>,
    pub transaction_index: Option<u64>,
    pub log_index: Option<u64>,
    pub removed: bool,
    pub transaction_log_index: Option<u64>,
    pub log_type: Option<String>,
}

impl From<Transaction> for WvmTransaction {
    fn from(tx: Transaction) -> Self {
        let mut other = ethers::types::OtherFields::default();
        Self {
            hash: tx.hash.0,
            nonce: {
                let mut bytes = [0u8; 32];
                tx.nonce.to_big_endian(&mut bytes);
                bytes
            },
            block_hash: tx.block_hash.map(|h| h.0),
            block_number: tx.block_number.map(|n| n.as_u64()),
            transaction_index: tx.transaction_index.map(|i| i.as_u64()),
            from: tx.from.0,
            to: tx.to.map(|a| a.0),
            value: {
                let mut bytes = [0u8; 32];
                tx.value.to_big_endian(&mut bytes);
                bytes
            },
            gas_price: tx.gas_price.map(|g| {
                let mut bytes = [0u8; 32];
                g.to_big_endian(&mut bytes);
                bytes
            }),
            gas: {
                let mut bytes = [0u8; 32];
                tx.gas.to_big_endian(&mut bytes);
                bytes
            },
            input: tx.input.to_vec(),
            v: tx.v.as_u64(),
            r: {
                let mut bytes = [0u8; 32];
                tx.r.to_big_endian(&mut bytes);
                bytes
            },
            s: {
                let mut bytes = [0u8; 32];
                tx.s.to_big_endian(&mut bytes);
                bytes
            },
            transaction_type: tx.transaction_type.map(|t| t.as_u64()),
            access_list: tx.access_list.map(|list| {
                list.0
                    .into_iter()
                    .map(|item| {
                        (
                            item.address.0.to_vec(),
                            item.storage_keys.into_iter().map(|k| k.0).collect(),
                        )
                    })
                    .collect()
            }),
            max_priority_fee_per_gas: tx.max_priority_fee_per_gas.map(|f| {
                let mut bytes = [0u8; 32];
                f.to_big_endian(&mut bytes);
                bytes
            }),
            max_fee_per_gas: tx.max_fee_per_gas.map(|f| {
                let mut bytes = [0u8; 32];
                f.to_big_endian(&mut bytes);
                bytes
            }),
            chain_id: tx.chain_id.map(|c| {
                let mut bytes = [0u8; 32];
                c.to_big_endian(&mut bytes);
                bytes
            }),
            source_hash: tx
                .other
                .get("sourceHash")
                .and_then(|v| v.as_str())
                .map(|h| H256::from_str(h).unwrap().0),
            mint: tx.other.get("mint").and_then(|v| v.as_str()).map(|m| {
                let val = U256::from_str(m).unwrap();
                let mut bytes = [0u8; 32];
                val.to_big_endian(&mut bytes);
                bytes
            }),
            is_system_tx: tx
                .other
                .get("isSystemTx")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            fee_currency: None,
            gateway_fee_recipient: None,
            gateway_fee: None,
        }
    }
}

impl From<WvmTransaction> for Transaction {
    fn from(tx: WvmTransaction) -> Self {
        let mut other = ethers::types::OtherFields::default();
        if let Some(source_hash) = tx.source_hash {
            other.insert(
                "sourceHash".to_string(),
                format!("0x{}", hex::encode(source_hash)).into(),
            );
        }
        if let Some(mint) = tx.mint {
            other.insert(
                "mint".to_string(),
                format!("0x{}", hex::encode(mint)).into(),
            );
        }
        if tx.is_system_tx {
            other.insert("isSystemTx".to_string(), "true".into());
        }

        Self {
            hash: H256::from(tx.hash),
            nonce: U256::from_big_endian(&tx.nonce),
            block_hash: tx.block_hash.map(H256::from),
            block_number: tx.block_number.map(U64::from),
            transaction_index: tx.transaction_index.map(U64::from),
            from: Address::from(tx.from),
            to: tx.to.map(Address::from),
            value: U256::from_big_endian(&tx.value),
            gas_price: tx.gas_price.map(|g| U256::from_big_endian(&g)),
            gas: U256::from_big_endian(&tx.gas),
            input: Bytes::from(tx.input),
            v: U64::from(tx.v),
            r: U256::from_big_endian(&tx.r),
            s: U256::from_big_endian(&tx.s),
            transaction_type: tx.transaction_type.map(U64::from),
            access_list: tx.access_list.map(|list| {
                AccessList(
                    list.into_iter()
                        .map(|(addr, keys)| AccessListItem {
                            address: Address::from_slice(&addr),
                            storage_keys: keys.into_iter().map(H256::from).collect(),
                        })
                        .collect(),
                )
            }),
            max_priority_fee_per_gas: tx
                .max_priority_fee_per_gas
                .map(|f| U256::from_big_endian(&f)),
            max_fee_per_gas: tx.max_fee_per_gas.map(|f| U256::from_big_endian(&f)),
            chain_id: tx.chain_id.map(|c| U256::from_big_endian(&c)),
            other,
        }
    }
}

impl From<TransactionReceipt> for WvmTransactionReceipt {
    fn from(receipt: TransactionReceipt) -> Self {
        Self {
            transaction_hash: receipt.transaction_hash.0,
            transaction_index: receipt.transaction_index.as_u64(),
            block_hash: receipt.block_hash.map(|h| h.0),
            block_number: receipt.block_number.map(|n| n.as_u64()),
            from: receipt.from.0,
            to: receipt.to.map(|a| a.0),
            cumulative_gas_used: {
                let mut bytes = [0u8; 32];
                receipt.cumulative_gas_used.to_big_endian(&mut bytes);
                bytes
            },
            gas_used: receipt.gas_used.map(|g| {
                let mut bytes = [0u8; 32];
                g.to_big_endian(&mut bytes);
                bytes
            }),
            contract_address: receipt.contract_address.map(|a| a.0),
            logs: receipt.logs.into_iter().map(Into::into).collect(),
            status: receipt.status.map(|s| s.as_u64()),
            root: receipt.root.map(|r| r.0),
            logs_bloom: Some(receipt.logs_bloom.0),
            transaction_type: receipt.transaction_type.map(|t| t.as_u64()),
            effective_gas_price: receipt.effective_gas_price.map(|p| {
                let mut bytes = [0u8; 32];
                p.to_big_endian(&mut bytes);
                bytes
            }),
            deposit_nonce: receipt
                .other
                .get("depositNonce")
                .and_then(|v| v.as_str())
                .map(|n| u64::from_str_radix(&n[2..], 16).unwrap()),
            l1_fee: None,
            l1_fee_scalar: None,
            l1_gas_price: None,
            l1_gas_used: None,
        }
    }
}

impl From<Withdrawal> for WvmWithdrawal {
    fn from(withdrawal: Withdrawal) -> Self {
        Self {
            index: withdrawal.index.as_u64(),
            validator_index: withdrawal.validator_index.as_u64(),
            address: withdrawal.address.0,
            amount: withdrawal.amount.as_u64(),
        }
    }
}

impl From<Block<Transaction>> for WvmBlock<WvmTransaction> {
    fn from(block: Block<Transaction>) -> Self {
        Self {
            hash: block.hash.map(|h| h.0),
            parent_hash: block.parent_hash.0,
            uncles_hash: block.uncles_hash.0,
            author: block.author.map(|a| a.0),
            state_root: block.state_root.0,
            transactions_root: block.transactions_root.0,
            receipts_root: block.receipts_root.0,
            number: block.number.map(|n| n.as_u64()),
            gas_used: {
                let mut bytes = [0u8; 32];
                block.gas_used.to_big_endian(&mut bytes);
                bytes
            },
            gas_limit: {
                let mut bytes = [0u8; 32];
                block.gas_limit.to_big_endian(&mut bytes);
                bytes
            },
            extra_data: block.extra_data.to_vec(),
            logs_bloom: block.logs_bloom.map(|b| b.0),
            timestamp: {
                let mut bytes = [0u8; 32];
                block.timestamp.to_big_endian(&mut bytes);
                bytes
            },
            difficulty: {
                let mut bytes = [0u8; 32];
                block.difficulty.to_big_endian(&mut bytes);
                bytes
            },
            total_difficulty: block.total_difficulty.map(|d| {
                let mut bytes = [0u8; 32];
                d.to_big_endian(&mut bytes);
                bytes
            }),
            seal_fields: block.seal_fields.into_iter().map(|f| f.to_vec()).collect(),
            uncles: block.uncles.into_iter().map(|u| u.0).collect(),
            transactions: block.transactions.into_iter().map(Into::into).collect(),
            size: block.size.map(|s| {
                let mut bytes = [0u8; 32];
                s.to_big_endian(&mut bytes);
                bytes
            }),
            mix_hash: block.mix_hash.map(|h| h.0),
            nonce: block.nonce.map(|n| n.0),
            base_fee_per_gas: block.base_fee_per_gas.map(|f| {
                let mut bytes = [0u8; 32];
                f.to_big_endian(&mut bytes);
                bytes
            }),
            blob_gas_used: block.blob_gas_used.map(|g| {
                let mut bytes = [0u8; 32];
                g.to_big_endian(&mut bytes);
                bytes
            }),
            excess_blob_gas: block.excess_blob_gas.map(|g| {
                let mut bytes = [0u8; 32];
                g.to_big_endian(&mut bytes);
                bytes
            }),
            withdrawals_root: block.withdrawals_root.map(|r| r.0),
            withdrawals: block
                .withdrawals
                .map(|w| w.into_iter().map(Into::into).collect()),
            parent_beacon_block_root: block.parent_beacon_block_root.map(|r| r.0),
        }
    }
}

impl From<WvmBlock<WvmTransaction>> for Block<Transaction> {
    fn from(wvm_block: WvmBlock<WvmTransaction>) -> Self {
        Self {
            hash: wvm_block.hash.map(H256::from),
            parent_hash: H256::from(wvm_block.parent_hash),
            uncles_hash: H256::from(wvm_block.uncles_hash),
            author: wvm_block.author.map(Address::from),
            state_root: H256::from(wvm_block.state_root),
            transactions_root: H256::from(wvm_block.transactions_root),
            receipts_root: H256::from(wvm_block.receipts_root),
            number: wvm_block.number.map(U64::from),
            gas_used: U256::from_big_endian(&wvm_block.gas_used),
            gas_limit: U256::from_big_endian(&wvm_block.gas_limit),
            extra_data: Bytes::from(wvm_block.extra_data),
            logs_bloom: wvm_block.logs_bloom.map(|b| ethers::types::Bloom::from(b)),
            timestamp: U256::from_big_endian(&wvm_block.timestamp),
            difficulty: U256::from_big_endian(&wvm_block.difficulty),
            total_difficulty: wvm_block
                .total_difficulty
                .map(|d| U256::from_big_endian(&d)),
            seal_fields: wvm_block.seal_fields.into_iter().map(Bytes::from).collect(),
            uncles: wvm_block.uncles.into_iter().map(H256::from).collect(),
            transactions: wvm_block.transactions.into_iter().map(Into::into).collect(),
            size: wvm_block.size.map(|s| U256::from_big_endian(&s)),
            mix_hash: wvm_block.mix_hash.map(H256::from),
            nonce: wvm_block.nonce.map(|n| ethers::types::H64::from(n)),
            base_fee_per_gas: wvm_block
                .base_fee_per_gas
                .map(|f| U256::from_big_endian(&f)),
            blob_gas_used: wvm_block.blob_gas_used.map(|g| U256::from_big_endian(&g)),
            excess_blob_gas: wvm_block.excess_blob_gas.map(|g| U256::from_big_endian(&g)),
            withdrawals_root: wvm_block.withdrawals_root.map(H256::from),
            withdrawals: wvm_block
                .withdrawals
                .map(|w| w.into_iter().map(Into::into).collect()),
            parent_beacon_block_root: wvm_block.parent_beacon_block_root.map(H256::from),
            other: Default::default(),
        }
    }
}

impl From<WvmWithdrawal> for Withdrawal {
    fn from(w: WvmWithdrawal) -> Self {
        Self {
            index: U64::from(w.index),
            validator_index: U64::from(w.validator_index),
            address: Address::from(w.address),
            amount: U256::from(w.amount),
        }
    }
}

impl From<WvmTransactionReceipt> for TransactionReceipt {
    fn from(receipt: WvmTransactionReceipt) -> Self {
        Self {
            transaction_hash: H256::from(receipt.transaction_hash),
            transaction_index: U64::from(receipt.transaction_index),
            block_hash: receipt.block_hash.map(H256::from),
            block_number: receipt.block_number.map(U64::from),
            from: Address::from(receipt.from),
            to: receipt.to.map(Address::from),
            cumulative_gas_used: U256::from_big_endian(&receipt.cumulative_gas_used),
            gas_used: receipt.gas_used.map(|g| U256::from_big_endian(&g)),
            contract_address: receipt.contract_address.map(Address::from),
            logs: receipt.logs.into_iter().map(Into::into).collect(),
            status: receipt.status.map(U64::from),
            root: receipt.root.map(H256::from),
            logs_bloom: ethers::types::Bloom::from(receipt.logs_bloom.unwrap()),
            transaction_type: receipt.transaction_type.map(U64::from),
            effective_gas_price: receipt
                .effective_gas_price
                .map(|p| U256::from_big_endian(&p)),
            other: {
                let mut other = ethers::types::OtherFields::default();
                if let Some(nonce) = receipt.deposit_nonce {
                    other.insert("depositNonce".to_string(), format!("0x{:x}", nonce).into());
                }
                other
            },
        }
    }
}

impl From<Log> for WvmLog {
    fn from(log: Log) -> Self {
        Self {
            address: log.address.0,
            topics: log.topics.into_iter().map(|t| t.0).collect(),
            data: log.data.to_vec(),
            block_hash: log.block_hash.map(|h| h.0),
            block_number: log.block_number.map(|n| n.as_u64()),
            transaction_hash: log.transaction_hash.map(|h| h.0),
            transaction_index: log.transaction_index.map(|i| i.as_u64()),
            log_index: log.log_index.map(|i| i.as_u64()),
            removed: log.removed.is_some(),
            transaction_log_index: log.transaction_log_index.map(|i| i.as_u64()),
            log_type: log.log_type,
        }
    }
}

impl From<WvmLog> for Log {
    fn from(log: WvmLog) -> Self {
        Self {
            address: Address::from(log.address),
            topics: log.topics.into_iter().map(H256::from).collect(),
            data: Bytes::from(log.data),
            block_hash: log.block_hash.map(H256::from),
            block_number: log.block_number.map(U64::from),
            transaction_hash: log.transaction_hash.map(H256::from),
            transaction_index: log.transaction_index.map(U64::from),
            log_index: log.log_index.map(U256::from),
            removed: Some(log.removed),
            transaction_log_index: log.transaction_log_index.map(U256::from),
            log_type: log.log_type,
        }
    }
}
