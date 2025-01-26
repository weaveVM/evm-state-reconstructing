use ethereum_types::{H160, H256, U256};
use revm::{
    db::InMemoryDB,
    primitives::{TransactTo, ExecutionResult, U256 as rU256, Address},
    Evm,
    Database
};
use ethers::types::{Transaction, Block, TransactionReceipt};
use std::collections::HashMap;
use crate::utils::core::genesis_load::Genesis;
use ethers::utils::rlp;

#[derive(Debug, Clone)]
pub struct AccountState {
    pub nonce: U256,
    pub balance: U256,
    pub storage: HashMap<H256, H256>,
    pub code: Vec<u8>,
}

pub struct StateReconstructor {
    pub evm: Evm<'static, (), InMemoryDB>,
    pub accounts: HashMap<H256, AccountState>,
    pub block_number: u64,
}

impl StateReconstructor {
    pub fn new() -> Self {
        let db = InMemoryDB::default();
        let evm = Evm::builder()
            .with_db(db)
            .modify_tx_env(|tx| {
                tx.gas_price = rU256::ZERO;
            })
            .modify_block_env(|block| {
                block.gas_limit = rU256::from(30_000_000);
            })
            .build();

        Self {
            evm,
            accounts: HashMap::new(),
            block_number: 0,
        }
    }

    pub fn apply_transaction(&mut self, tx: Transaction) -> Result<(), Box<dyn std::error::Error>> {
        let from_addr = tx.from;
        let to_addr = tx.to;

        self.evm.tx_mut().caller = from_addr.0.into();
        self.evm.tx_mut().gas_price = rU256::from(tx.gas_price.unwrap_or_default().as_u64());
        self.evm.tx_mut().gas_limit = tx.gas.as_u64();
        self.evm.tx_mut().transact_to = to_addr.map(|t| TransactTo::Call(t.0.into())).unwrap_or(TransactTo::Create);
        self.evm.tx_mut().value = rU256::from(tx.value.as_u64());
        self.evm.tx_mut().data = tx.input.to_vec().into();

        let result = self.evm.transact()?.result;

        match result {
            ExecutionResult::Success { .. } => {
                // Update sender account
                let from_converted: Address = Address::from_slice(&from_addr.0);
                let from_info = self.evm.db_mut().basic(from_converted)?;
                if let Some(account_info) = from_info {
                    self.accounts.insert(
                        from_addr.into(),
                        AccountState {
                            nonce: account_info.nonce.into(),
                            balance: U256::from_big_endian(&account_info.balance.to_be_bytes::<32>()),
                            storage: HashMap::new(),
                            code: account_info.code.map(|code| code.bytecode().to_vec()).unwrap_or_default(),
                        },
                    );
                }

                // Update recipient account (if any)
                if let Some(to_addr) = to_addr {
                    let to_converted: Address = Address::from_slice(&to_addr.0);
                    let to_info = self.evm.db_mut().basic(to_converted)?;
                    if let Some(account_info) = to_info {
                        self.accounts.insert(
                            to_addr.into(),
                            AccountState {
                                nonce: account_info.nonce.into(),
                                balance: U256::from_big_endian(&account_info.balance.to_be_bytes::<32>()),
                                storage: HashMap::new(),
                                code: account_info.code.map(|code| code.bytecode().to_vec()).unwrap_or_default(),
                            },
                        );
                    }
                }

                // Handle contract creation
                if to_addr.is_none() {
                    // Derive the contract address
                    let sender_nonce = {
                        let db = self.evm.db_mut(); // Immutable borrow
                        db.basic(from_addr.0.into())?.unwrap().nonce
                    };
                    let contract_address = H160::from_slice(
                        &revm::primitives::keccak256(
                            &rlp::encode_list::<&[u8], &[u8]>(&[
                                &from_addr.0.as_ref(), // Convert [u8; 20] to &[u8]
                                &sender_nonce.to_be_bytes().as_ref(), // Convert u64 to &[u8]
                            ]),
                        )[12..],
                    );
                    // Insert the new contract's account state
                    let contract_info = self.evm.db_mut().basic(Address::from_slice(&contract_address.0))?;
                    if let Some(account_info) = contract_info {
                        self.accounts.insert(
                            H256::from_slice(&contract_address.0),
                            AccountState {
                                nonce: account_info.nonce.into(),
                                balance: U256::from_big_endian(&account_info.balance.to_be_bytes::<32>()),
                                storage: HashMap::new(),
                                code: account_info.code.map(|code| code.bytecode().to_vec()).unwrap_or_default(),
                            },
                        );
                    }
                }
            }
            ExecutionResult::Revert { .. } => {
                println!("Transaction reverted");
            }
            ExecutionResult::Halt { .. } => {
                println!("Transaction halted");
            }
        }

        Ok(())
    }

    pub fn apply_block(&mut self, block: &Block<Transaction>, receipts: &[TransactionReceipt]) -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            block.transactions.len(),
            receipts.len(),
            "mismatched transaction and receipt counts"
        );

        for (tx, receipt) in block.transactions.iter().zip(receipts.iter()) {
            self.apply_transaction(tx.clone())?;
        }

        self.block_number = block.number.unwrap_or_default().as_u64();
        Ok(())
    }

    pub fn get_account_state(&self, address: H256) -> Option<&AccountState> {
        self.accounts.get(&address)
    }

    pub fn from_genesis(genesis: &Genesis) -> Self {
        let db = InMemoryDB::default();
        let evm = Evm::builder()
            .with_db(db)
            .modify_tx_env(|tx| {
                tx.gas_price = rU256::ZERO;
            })
            .modify_block_env(|block| {
                block.gas_limit = rU256::from_str_radix(&genesis.gas_limit.trim_start_matches("0x"), 16)
                    .unwrap_or(rU256::from(30_000_000));
                block.number = rU256::from_str_radix(&genesis.number.trim_start_matches("0x"), 16)
                    .unwrap_or_default();
                block.timestamp = rU256::from_str_radix(&genesis.timestamp.trim_start_matches("0x"), 16)
                    .unwrap_or_default();
                block.coinbase = genesis.coinbase.0.into();
                block.difficulty = rU256::from_str_radix(&genesis.difficulty.trim_start_matches("0x"), 16)
                    .unwrap_or_default();
                block.basefee = rU256::from_str_radix(&genesis.base_fee_per_gas.trim_start_matches("0x"), 16)
                    .unwrap_or_default();
            })
            .build();

        let mut state = Self {
            evm,
            accounts: HashMap::new(),
            block_number: 0,
        };

        // Initialize accounts
        for (address, alloc) in &genesis.alloc {
            let balance = rU256::from_str_radix(&alloc.balance, 10).unwrap_or(rU256::ZERO);
            let code = hex::decode(alloc.code.strip_prefix("0x").unwrap_or("")).unwrap_or_default();
            let nonce = rU256::from_str_radix(&alloc.nonce.trim_start_matches("0x"), 16)
                .unwrap_or(rU256::ZERO).to::<u64>();

            for (key, value) in &alloc.storage {
                let _ = state.evm.db_mut().insert_account_storage(
                    address.0.into(),
                    rU256::from_be_bytes(key.0),
                    rU256::from_be_bytes(value.0),
                );
            }

            state.evm.db_mut().insert_account_info(
                address.0.into(),
                revm::primitives::AccountInfo {
                    balance,
                    nonce: nonce.into(),
                    code: Some(revm::primitives::Bytecode::new_raw(code.into())),
                    ..Default::default()
                },
            );
        }

        state
    }
}