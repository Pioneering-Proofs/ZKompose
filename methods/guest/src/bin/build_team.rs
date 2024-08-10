use std::io::Read;

use alloy_primitives::{address, Address, U256};
use alloy_sol_types::{sol, SolValue};
use json::parse;
use risc0_zkvm::guest::env;

use common::types::{Player, PlayerData, Skills, Team};

/// Alloy interface to use for verifying on-chain state
sol! {
    interface IERC721 {
        function tokenURI(uint256 tokenId) external view returns (string memory);
        function ownerOf(uint256 tokenId) external view returns (address owner);
    }
}

fn main() {
    // TODO: Signature of owner will need to be provided to ensure this operation is authorized
    let data: String = env::read();
    let data = parse(&data).unwrap();
    println!("Team name: {}", data["name"].as_str().unwrap());

    env::commit_slice(U256::from(42).abi_encode().as_slice());
}
