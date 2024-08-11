use std::io::Read;

use alloy_primitives::{address, Address, U256};
use alloy_sol_types::{sol, SolValue};
use common::{
    team,
    types::{GenTeamInput, Player, PlayerPosition, Roster, Skills, Team},
};
use json::parse;
use risc0_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC,
    ethereum::{EthEvmEnv, EthEvmInput},
    Contract, EvmBlockHeader, SolCommitment,
};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

/// Alloy interface to use for verifying on-chain state
sol! {
    interface IERC721 {
        function tokenURI(uint256 tokenId) external view returns (string memory);
        function ownerOf(uint256 tokenId) external view returns (address owner);
    }

    struct Journal {
        SolCommitment commitment;
        bytes32 teamCID;
        uint256[11] playerIds;
    }
}

const PLAYER_CONTRACT_ADDRESS: Address = address!("74bf0b8f065aa2627ea25c6a07cba79407ae7265");
const TEAM_CONTRACT_ADDRESS: Address = address!("b507B268BF896a65Eb1B9b4176eD65ab4a91604D");

fn main() {
    // TODO: Signature of owner will need to be provided to ensure this operation is authorized
    let chain_config: EthEvmInput = env::read();
    let input: GenTeamInput = env::read();

    let env = chain_config
        .into_env()
        .with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);

    let team = Team::new(input.roster.clone(), input.name, input.logo_uri);
    let team_cid = team.cid();

    // 1. Validate caller is owner of all players
    // let contract = Contract::new(PLAYER_CONTRACT_ADDRESS, &env);
    // roster.into_iter().for_each(|player| match player {
    //     PlayerPosition::Goalie(player) => contract
    //         .clone()
    //         .call_builder(IERC721::ownerOfCall {
    //             tokenId: player.token_id,
    //         })
    //         .call(),
    //     PlayerPosition::Defense(player, _) => contract
    //         .clone()
    //         .call_builder(IERC721::ownerOfCall {
    //             tokenId: player.token_id,
    //         })
    //         .call(),
    //     PlayerPosition::Mid(player, _) => contract
    //         .clone()
    //         .call_builder(IERC721::ownerOfCall {
    //             tokenId: player.token_id,
    //         })
    //         .call(),
    //     PlayerPosition::Offense(player, _) => contract
    //         .clone()
    //         .call_builder(IERC721::ownerOfCall {
    //             tokenId: player.token_id,
    //         })
    //         .call(),
    // });

    env::commit_slice(&U256::from(1).abi_encode());

    // let journal = Journal {
    //     commitment: chain_config.block_commitment(),
    //     teamCID: 0x0,
    //     playerIds: roster.into_iter().map(|player| player.token_id).collect(),
    // };
    // env::commit_slice(&journal.abi_encode());
}
