use alloy_sol_types::{sol, SolValue};
use array_init::array_init;
use common::{
    constants::{PLAYER_BATCH_SIZE, PLAYER_STD_DEV},
    math::new_u_v,
    types::Player,
    utils::match_player_tier,
};
use risc0_zkvm::guest::env;
use std::io::Read;

sol! {
    struct Input {
        uint8 tier;
        uint256 order_id;
        uint256 u;
        uint256 v;
    }

    struct Journal {
        uint8 tier;
        uint256 order_id;
        bytes32[15] cids;
    }
}

fn main() {
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    let input: Input = <Input>::abi_decode(&input_bytes, true).unwrap();

    let median = match_player_tier(input.tier);
    let mut u: f64 = input.u.to::<u64>() as f64;
    let mut v: f64 = input.v.to::<u64>() as f64;

    let players: [Player; PLAYER_BATCH_SIZE] = array_init(|i| {
        let player = Player::new(
            (input.order_id.to::<u32>() * PLAYER_BATCH_SIZE as u32) + i as u32,
            PLAYER_STD_DEV,
            median,
            u,
            v,
        );
        (u, v) = new_u_v(u, v);
        player
    });
    let cids: [[u8; 32]; PLAYER_BATCH_SIZE] = array_init(|i| {
        let player = players[i].cid();
        let bytes: [u8; 34] = player.try_into().expect("CID is not 46 bytes");
        bytes[2..].try_into().expect("CID is not 32 bytes")
    });

    let journal = Journal {
        tier: input.tier,
        order_id: input.order_id,
        cids: cids.map(|bytes| bytes.into()),
    };

    env::write(&players);
    env::commit_slice(journal.abi_encode().as_slice());
}
