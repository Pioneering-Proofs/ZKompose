use common::types::{GenPlayersInput, Player};
use risc0_zkvm::guest::env;

fn main() {
    let input: GenPlayersInput = env::read();

    let players: [String; 15] = array_init::array_init(|i| {
        Player::new(
            (input.order_id * 15) + i as u32,
            input.std_dev,
            input.median,
            input.u,
            input.v,
        )
        .cid()
    });
    env::commit(&players)
}
