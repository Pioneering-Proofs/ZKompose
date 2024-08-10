use std::io::Read;

use alloy_primitives::U256;
use alloy_sol_types::SolValue;
use risc0_zkvm::guest::env;

fn main() {
    env::commit_slice(U256::from(42).abi_encode().as_slice());
}
