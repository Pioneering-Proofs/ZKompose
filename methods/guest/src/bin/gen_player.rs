use alloy_primitives::{U256, U8};
use common::{
    math::new_u_v,
    types::{GenPlayersInput, GenPlayersJournal, Player},
};
use risc0_zkvm::guest::env;

fn match_player_tier(tier: u8) -> u8 {
    match tier {
        0 => 90,
        1 => 80,
        2 => 70,
        3 => 60,
        4 => 60,
        _ => 60,
    }
}

fn main() {
    let input: GenPlayersInput = env::read();

    let median = match_player_tier(input.tier);
    let mut u = input.u;
    let mut v = input.v;

    let mut players: [String; 15] = Default::default();

    for i in 0..15 {
        (u, v) = new_u_v(u, v);
        players[i] = Player::new(
            (input.order_id * 15) + i as u32,
            input.std_dev,
            median,
            u,
            v,
        )
        .cid();
    }

    let journal = GenPlayersJournal {
        tier: U8::from(input.tier),
        order_id: U256::from(input.order_id),
        cids: players,
    };
    env::commit(&journal);
}
