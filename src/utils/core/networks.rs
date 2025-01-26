use crate::utils::constants::{PHALA_RPC_URL, WVM_RPC_URL};
use crate::utils::core::genesis_load::{load_genesis_from_file, Genesis};
use ethers::providers::{Http, Provider};

#[derive(Debug, Clone)]
pub struct Networks {
    pub rpc_url: String,
    pub genesis_file: Genesis,
    pub rpc_provider: Provider<Http>
}

impl Networks {
    pub fn weavevm() -> Networks {
        Self {
            rpc_url: WVM_RPC_URL.to_string(),
            genesis_file: load_genesis_from_file("./genesis/wvm_alphanet.json"),
            rpc_provider: Provider::<Http>::try_from(WVM_RPC_URL).unwrap()
        }
    }

    pub fn phala() -> Networks {
        Self {
            rpc_url: PHALA_RPC_URL.to_string(),
            genesis_file: load_genesis_from_file("./genesis/phala_mainnet.json"),
            rpc_provider: Provider::<Http>::try_from(PHALA_RPC_URL).unwrap()

        }
    }
}
