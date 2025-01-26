use ethers::types::{H160, H256};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Genesis {
    pub alloc: HashMap<H160, AccountAlloc>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct AccountAlloc {
    #[serde(default)]
    pub balance: String,
    #[serde(default)]
    pub code: String, // Contract code
    #[serde(default)]
    pub storage: HashMap<H256, H256>, // storage as KV pairs
}

pub fn load_genesis_from_file(path: &str) -> Genesis {
    let mut file = File::open(path).expect("Failed to open genesis file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read genesis.json");
    let genesis: Genesis = serde_json::from_str(&contents).expect("Failed to parse genesis.json");
    genesis
}
