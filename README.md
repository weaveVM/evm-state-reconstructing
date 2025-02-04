<p align="center">
  <a href="https://wvm.dev">
    <img src="https://raw.githubusercontent.com/weaveVM/.github/main/profile/bg.png">
  </a>
</p>

## About
EVM State Reconstruction is a PoC tool for testing purposes that demonstrates how to use EVM network data archived by an [wvm-archiver](https://github.com/weaveVM/wvm-archiver) instance to reconstruct its state trustlessly by pulling data from [WeaveVM](https://wvm.dev).

This PoC implements a simplified EVM state reconstruction logic using revm -- for production purposes users are free to implement their own logic in [evm_exec.rs](./src/utils/core/evm_exec.rs)

## Usage
Add it to your codebase:

```Cargo.toml
evm_state_reconstructing = {git = "https://github.com/weaveVM/evm-state-reconstructing", branch = "main"}
```

### Code example

Case example: Reconstructing the state of [RSS3 VSL Mainnet](https://rss3.io) :

```rust
use evm_state_reconstructing::utils::core::evm_exec::StateReconstructor;
use evm_state_reconstructing::utils::core::networks::Networks;
use evm_state_reconstructing::utils::core::reconstruct::reconstruct_network;
use anyhow::Error;


async fn reconstruct_state() -> Result<StateReconstructor, Error> {
    let network: Networks = Networks::rss3();
    let state: StateReconstructor = reconstruct_network(network).await?;
    Ok(state)
}
```

## License
This repository is licensed under the [MIT License](./LICENSE)