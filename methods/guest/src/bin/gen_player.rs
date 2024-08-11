use common::{
    math::new_u_v,
    types::{GenPlayersInput, Player},
};
use risc0_zkvm::guest::env;

fn main() {
    let input: GenPlayersInput = env::read();

    let mut u = input.u;
    let mut v = input.v;

    let mut players: [String; 15] = Default::default();

    for i in 0..15 {
        (u, v) = new_u_v(u, v);
        players[i] = Player::new(
            (input.order_id * 15) + i as u32,
            input.std_dev,
            input.median,
            u,
            v,
        )
        .cid();
    }
    env::commit(&players)
}
