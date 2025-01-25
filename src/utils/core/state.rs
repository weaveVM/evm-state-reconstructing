use ethereum_types::{H256, U256, H160};
use ethers::types::{Block, Transaction, TransactionReceipt};
use std::collections::HashMap;
use crate::utils::core::genesis_load::Genesis;


#[derive(Debug, Clone)]
pub struct AccountState {
    pub nonce: U256,
    pub balance: U256,
    pub storage: HashMap<H256, H256>,
    pub code: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct StateReconstructor {
    pub accounts: HashMap<H256, AccountState>,
    pub block_number: u64,
}

impl StateReconstructor {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            block_number: 0,
        }
    }

    pub fn apply_transaction(&mut self, tx: &Transaction, receipt: &TransactionReceipt) {
        // update sender account
        if let from = tx.from {
            let sender = self
                .accounts
                .entry(H256::from(from))
                .or_insert(AccountState {
                    nonce: U256::zero(),
                    balance: U256::zero(), 
                    storage: HashMap::new(),
                    code: Vec::new(),
                });
    
            let gas_cost =
                tx.gas_price.unwrap_or_default() * U256::from(receipt.gas_used.unwrap_or_default());
    
            // Verify the sender has enough balance
            if sender.balance >= gas_cost + tx.value {
                sender.balance -= gas_cost + tx.value;
                sender.nonce += U256::one();
            } else {
                panic!(
                    "Insufficient balance: Balance is {:?}, but transaction requires {:?} (value: {:?}, gas cost: {:?})",
                    sender.balance, gas_cost + tx.value, tx.value, gas_cost
                );
            }
        }
    
        // update recipient account
        if let Some(to) = tx.to {
            let recipient = self.accounts.entry(H256::from(to)).or_insert(AccountState {
                nonce: U256::zero(),
                balance: U256::zero(),
                storage: HashMap::new(),
                code: Vec::new(),
            });
    
            recipient.balance += tx.value;
    
            // if the transaction deploys a contract, set its code
            if tx.input.len() > 0 {
                recipient.code = tx.input.to_vec();
            }
        }
    
        // update storage based on transaction logs
        for log in &receipt.logs {
            let account = self
                .accounts
                .entry(H256::from(log.address))
                .or_insert(AccountState {
                    nonce: U256::zero(),
                    balance: U256::zero(),
                    storage: HashMap::new(),
                    code: Vec::new(),
                });
    
            if log.topics.len() >= 2 {
                let storage_key = log.topics[0];
                let storage_value = log.topics[1];
                account.storage.insert(storage_key, storage_value);
            }
        }
    }
    

    pub fn apply_block(&mut self, block: &Block<Transaction>, receipts: &[TransactionReceipt]) {
        assert_eq!(
            block.transactions.len(),
            receipts.len(),
            "mismatched transaction and receipt counts"
        );

        for (tx, receipt) in block.transactions.iter().zip(receipts.iter()) {
            self.apply_transaction(tx, receipt);
        }

        self.block_number = block.number.unwrap_or_default().as_u64();
    }

    pub fn get_account_state(&self, address: H256) -> Option<&AccountState> {
        self.accounts.get(&address)
    }

    pub fn initialize_from_genesis(&mut self, genesis: Genesis) {
        for (address, alloc) in genesis.alloc {
            let balance = U256::from_dec_str(&alloc.balance).unwrap_or_else(|_| U256::zero());
            let code = hex::decode(alloc.code.strip_prefix("0x").unwrap_or("")).unwrap_or_else(|_| vec![]);
            let storage = alloc.storage;

            self.accounts.insert(
                H256::from(address),
                AccountState {
                    nonce: U256::zero(),
                    balance,
                    storage,
                    code,
                },
            );
        }
    }
}
