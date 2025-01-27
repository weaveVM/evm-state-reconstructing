use ethers::types::{H160, H256};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Genesis {
    pub config: GenesisConfig,
    pub alloc: HashMap<H160, AccountAlloc>,
    pub coinbase: H160,
    pub difficulty: String,
    pub extra_data: String,
    pub gas_limit: String,
    pub nonce: String,
    #[serde(default)]
    pub mixhash: String,
    #[serde(default)]
    pub parent_hash: String,
    pub timestamp: String,
    pub number: String,
    pub gas_used: String,
    pub base_fee_per_gas: String,
    pub excess_blob_gas: Option<String>,
    pub blob_gas_used: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenesisConfig {
    pub chain_id: u64,
    pub homestead_block: u64,
    pub eip150_block: u64,
    pub eip155_block: u64,
    pub eip158_block: u64,
    pub byzantium_block: u64,
    pub constantinople_block: u64,
    pub petersburg_block: u64,
    pub istanbul_block: u64,
    pub berlin_block: u64,
    pub london_block: u64,
    pub merge_netsplit_block: u64,
    pub terminal_total_difficulty: u64,
    pub terminal_total_difficulty_passed: bool,
    pub shanghai_time: u64,
    pub cancun_time: u64,
    #[serde(default)]
    pub deposit_contract_address: H160,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct AccountAlloc {
    #[serde(default)]
    pub balance: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub storage: HashMap<H256, H256>,
    #[serde(default)]
    pub nonce: String,
}

pub fn load_genesis_from_file(path: &str) -> Genesis {
    let mut file = File::open(path).expect("Failed to open genesis file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read genesis.json");
    let genesis: Genesis = serde_json::from_str(&contents).expect("Failed to parse genesis.json");
    genesis
}
