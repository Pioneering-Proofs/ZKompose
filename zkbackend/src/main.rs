use common::math::new_u_v;
use common::types::{GenPlayersInput, Player, PlayerJson};
use risc0_zkvm::{default_executor, serde::from_slice, ExecutorEnv};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post, routes};

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct PlayerRequestBody {
    u: f64,
    v: f64,
    tier: u8,
    order_id: u32,
}

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

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<req_body>")]
fn gen_player(req_body: Json<PlayerRequestBody>) -> Json<Vec<PlayerJson>> {
    let input = GenPlayersInput {
        order_id: req_body.order_id,
        buyer_pubkey: "".to_string(),
        std_dev: 5,
        tier: req_body.tier,
        u: req_body.u,
        v: req_body.v,
    };
    let mut players: Vec<PlayerJson> = vec![];

    let mut u = input.u;
    let mut v = input.v;

    for i in 0..15 {
        (u, v) = new_u_v(u, v);
        players.push(
            Player::new(
                (input.order_id * 15) + i as u32,
                input.std_dev,
                match_player_tier(input.tier),
                u,
                v,
            )
            .to_json(),
        );
    }

    println!("Input: {:?}", input);

    let env = ExecutorEnv::builder()
        .write(&input)
        .expect("Invalid input")
        .build()
        .unwrap();

    let session_info = default_executor()
        .execute(env, methods::GEN_PLAYER_ELF)
        .unwrap();

    let cids: [String; 15] =
        from_slice(&session_info.journal.bytes).expect("Failed to decode players from guest");

    for cid in cids.iter() {
        println!("CID: {:?}", cid);
    }

    Json(players)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/gen-player", routes![gen_player]);

    Ok(rocket.into())
}
