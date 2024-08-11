use common::{
    math::{calculate_tier, generate_overall_rating, generate_skill_scores, hash_f64, hash_i32},
    types::{Attribute, GenPlayersInput, PlayerJson, Skill},
};
use risc0_zkvm::guest::env;

fn gen_player(player_index: usize, std_dev: u8, median: u8, u: f64, v: f64) -> PlayerJson {
    let overall_rating = generate_overall_rating(std_dev as f64, median as f64, u, v);
    let skill_scores = generate_skill_scores(overall_rating, u, v);
    let u = hash_f64(u, None, None);
    let v = hash_f64(v, None, None);
    let jersey_number: u8 = hash_i32(u + v, Some(0), Some(99)) as u8;
    let name: String = PlayerJson::random_name(hash_i32(u + v, Some(0), Some(24)));

    // TODO: skibido
    // let player_svg = fs::read_to_string(format!("./player_svgs/{}.svg", player_index))?;
    // let svg_hash = Hash::of(player_svg.as_bytes()).await?;

    let player = PlayerJson {
        name: name.clone(),
        jersey_number,
        description: format!(
            "Number {} {}. Overall rating of {}.",
            jersey_number, name, overall_rating
        ),
        external_url: format!("https://example.com/player/{}", player_index),
        // image: format!("ipfs://{}", svg_hash),
        image: format!("ipfs://{}", "TODO: IPFS SVG HASH"),
        tier: calculate_tier(overall_rating),
        overall_rating,
        skill_multiplier: 1.0,
        skill: Skill {
            speed: skill_scores[0],
            shooting: skill_scores[1],
            passing: skill_scores[2],
            dribbling: skill_scores[3],
            defense: skill_scores[4],
            physical: skill_scores[5],
            goal_tending: skill_scores[6],
        },
        attributes: vec![
            Attribute {
                display_type: "skill_multiplier".to_string(),
                trait_type: "Skill Multiplier".to_string(),
                value: 1,
            },
            Attribute {
                display_type: "speed".to_string(),
                trait_type: "Speed".to_string(),
                value: skill_scores[0],
            },
            Attribute {
                display_type: "shooting".to_string(),
                trait_type: "Shooting".to_string(),
                value: skill_scores[1],
            },
            Attribute {
                display_type: "passing".to_string(),
                trait_type: "Passing".to_string(),
                value: skill_scores[2],
            },
            Attribute {
                display_type: "dribbling".to_string(),
                trait_type: "Dribbling".to_string(),
                value: skill_scores[3],
            },
            Attribute {
                display_type: "defense".to_string(),
                trait_type: "Defense".to_string(),
                value: skill_scores[4],
            },
            Attribute {
                display_type: "physical".to_string(),
                trait_type: "Physical".to_string(),
                value: skill_scores[5],
            },
            Attribute {
                display_type: "goal_tending".to_string(),
                trait_type: "Goal Tending".to_string(),
                value: skill_scores[6],
            },
        ],
    };

    player
}

fn gen_players(std_dev: u8, median: u8, u: f64, v: f64) -> [PlayerJson; 15] {
    array_init::array_init(|i| {
        gen_player(
            i,
            std_dev,
            median,
            hash_f64(u + i as f64, None, None),
            hash_f64(v + i as f64, None, None),
        )
    })
}

fn main() {
    let input: GenPlayersInput = env::read();

    let players = gen_players(input.std_dev, input.median, input.u, input.v);
    env::commit(&players)
}
