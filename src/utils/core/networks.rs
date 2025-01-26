use crate::utils::constants::{PHALA_RPC_URL, WVM_RPC_URL};
use crate::utils::core::genesis_load::{load_genesis_from_file, Genesis};

#[derive(Debug, Default, Clone)]
pub struct Network {
    pub rpc_url: String,
    pub genesis_file: Genesis,
}

impl Network {
    pub fn weavevm() -> Network {
        Self {
            rpc_url: WVM_RPC_URL.to_string(),
            genesis_file: load_genesis_from_file("./genesis/wvm_alphanet.json"),
        }
    }

    pub fn phala() -> Network {
        Self {
            rpc_url: PHALA_RPC_URL.to_string(),
            genesis_file: load_genesis_from_file("./genesis/phala_mainnet.json"),
        }
    }
}
