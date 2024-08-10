use common::types::{Attribute, GenPlayersInput, PlayerJson, Skill};
use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};

fn random_name(seed: i32) -> String {
    let first_name: [&str; 25] = [
        "James", "John", "Robert", "Michael", "William", "David", "Richard", "Joseph", "Thomas",
        "Charles", "Daniel", "Matthew", "Anthony", "Donald", "Mark", "Paul", "Steven", "Andrew",
        "Kenneth", "Joshua", "George", "Kevin", "Brian", "Edward", "Ronald",
    ];
    let last_name: [&str; 25] = [
        "Smith",
        "Johnson",
        "Williams",
        "Jones",
        "Brown",
        "Davis",
        "Miller",
        "Wilson",
        "Moore",
        "Taylor",
        "Anderson",
        "Thomas",
        "Jackson",
        "White",
        "Harris",
        "Martin",
        "Thompson",
        "Garcia",
        "Martinez",
        "Robinson",
        "Clark",
        "Rodriguez",
        "Lewis",
        "Lee",
        "Walker",
    ];

    let first_name_index = seed % first_name.len() as i32;
    let last_name_index = seed % last_name.len() as i32;

    format!(
        "{} {}",
        first_name[first_name_index as usize], last_name[last_name_index as usize]
    )
}

fn hash_f64(hash_u: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(1.0);
    let mut hasher = Sha256::new();
    hasher.update(hash_u.to_be_bytes());
    min + hasher
        .finalize()
        .as_slice()
        .iter()
        .fold(0.0, |acc, &x| acc + x as f64)
        % (max - min)
}

fn hash_i32(hash_u: f64, min: Option<i32>, max: Option<i32>) -> i32 {
    let min = min.unwrap_or(0);
    let max = max.unwrap_or(1);
    let mut hasher = Sha256::new();
    hasher.update(hash_u.to_be_bytes());
    min + hasher
        .finalize()
        .as_slice()
        .iter()
        .fold(0, |acc, &x| acc + x as i32)
        % (max - min)
}

fn f64_to_u8(f: f64) -> u8 {
    f.round() as u8
}

fn normal_random(u: f64, v: f64) -> f64 {
    -(2.0 * u.ln()).sqrt() * (2.0 * std::f64::consts::PI * v).cos()
}

fn generate_overall_rating(std_dev: f64, median: f64, u: f64, v: f64) -> u8 {
    let rating = (normal_random(u, v) * std_dev + median).round() as i32;
    rating.max(40).min(99) as u8
}

fn generate_skill_scores(overall_rating: u8, u: f64, v: f64) -> Vec<u8> {
    let base_skill = (overall_rating as i32 - 20).max(1) as u8;
    let skill_spread = ((overall_rating - base_skill) / 2) as u8;
    let mut u = u;
    let mut v = v;

    let mut skills: Vec<u8> = (0..7)
        .map(|_| {
            // let skill = base_skill as i32 + rng.gen_range(0..=skill_spread as i32 * 2);
            let skill = hash_i32(u + v, Some(0), Some(skill_spread as i32));
            let skill = base_skill as i32 + skill;
            u = hash_f64(u, None, None);
            v = hash_f64(v, None, None);

            skill.max(1).min(99) as u8
        })
        .collect();

    // let primary_skill_index = rng.gen_range(0..skills.len());
    let primary_skill_index = hash_i32(u + v, Some(0), Some(skills.len() as i32)) as usize;

    u = hash_f64(u, None, None);
    v = hash_f64(v, None, None);

    skills[primary_skill_index] = overall_rating - hash_i32(u + v, Some(0), Some(5)) as u8;

    skills
}

fn calculate_tier(overall_rating: u8) -> u8 {
    match overall_rating {
        90..=99 => 0, // Superstar
        80..=89 => 1, // All-Star
        70..=79 => 2, // Starter
        60..=69 => 3, // Rotation
        _ => 4,       // Bench
    }
}

fn gen_player(player_index: usize, std_dev: u8, median: u8, u: f64, v: f64) -> PlayerJson {
    let overall_rating = generate_overall_rating(std_dev as f64, median as f64, u, v);
    let skill_scores = generate_skill_scores(overall_rating, u, v);
    let u = hash_f64(u, None, None);
    let v = hash_f64(v, None, None);
    let jersey_number: u8 = hash_i32(u + v, Some(0), Some(99)) as u8;
    let name: String = random_name(hash_i32(u + v, Some(0), Some(24)));

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

fn gen_players(player_count: usize, std_dev: u8, median: u8, u: f64, v: f64) -> Vec<PlayerJson> {
    // create a Sha256 object
    let mut hash_u = u;
    let mut hash_v = v;

    let mut players = Vec::new();

    for i in 0..player_count {
        hash_u = hash_f64(hash_u, None, None);
        hash_v = hash_f64(hash_v, None, None);

        players.push(gen_player(i, std_dev, median, hash_u, hash_v));
    }

    players
}

fn main() {
    let input: GenPlayersInput = env::read();

    let players = gen_players(
        input.player_count,
        input.std_dev,
        input.median,
        input.u,
        input.v,
    );
    env::commit(&players)
}
