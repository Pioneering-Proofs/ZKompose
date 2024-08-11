use alloy_primitives::{FixedBytes, U256, U8};
use alloy_sol_types::{sol, SolValue};
use array_init::array_init;
use common::{
    math::new_u_v,
    types::{GenPlayersInput, GenPlayersJournal, Player},
    utils::match_player_tier,
};
use risc0_zkvm::guest::env;
use std::io::Read;

sol! {
    struct Input {
        uint8 tier;
        uint256 order_id;
        uint8 std_dev;
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
    let mut u = input.u;
    let mut v = input.v;

    let mut cids: [[u8; 32]; 15] = array_init(|_| [0; 32]);

    for i in 0..15 {
        let cid: [u8; 46] = Player::new(
            (input.order_id.to::<u32>() * 15) + i as u32,
            10,
            median,
            u.to::<u64>() as f64,
            v.to::<u64>() as f64,
        )
        .cid()
        .as_bytes()
        .try_into()
        .expect("CID is not 46 bytes");
        for j in 0..32 {
            cids[i][j] = cid[j + 13];
        }
    }

    let journal = Journal {
        tier: input.tier,
        order_id: input.order_id,
        cids: cids.map(|cid| cid.into()),
    };
    env::commit_slice(journal.abi_encode().as_slice()); // &ys[1 .. 4]
                                                        // env::commit_slice(U8::from(input.tier).abi_encode().as_slice());
                                                        // env::commit_slice(journal.abi_encode().as_slice());
                                                        // env::commit(&journal);
}
