use alloy_primitives::U256;
use alloy_sol_types::{private::Address, sol, SolValue};
use common::{constants::PLAYER_CONTRACT_ADDRESS, types::Player};
use risc0_steel::{config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthEvmInput, Contract};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

sol! {
    interface IERC721 {
        function tokenURI(uint256 tokenId) external view returns (string memory uri);
        function ownerOf(uint256 tokenId) external view returns (address owner);
    }
}

fn main() {
    let chain_config: EthEvmInput = env::read();
    let player: Player = env::read();

    let env = chain_config
        .into_env()
        .with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);

    let contract_address = Address::from_slice(PLAYER_CONTRACT_ADDRESS.to_vec().as_slice());
    let contract = Contract::new(contract_address, &env);

    let owner_call = IERC721::ownerOfCall {
        tokenId: U256::from(player.token_id),
    };
    let owner = contract.call_builder(&owner_call).call().owner;

    let player_cid_call = IERC721::tokenURICall {
        tokenId: U256::from(player.token_id),
    };
    let player_cid = contract.call_builder(&player_cid_call).call().uri;

    let expected_cid = player.cid_string().expect("Player CID is not valid");
    assert!(
        expected_cid == player_cid,
        "Player CID does not match on-chain data"
    );

    env::commit_slice(&env.block_commitment().abi_encode());
    env::commit(&owner);
    env::commit(&player);
}
