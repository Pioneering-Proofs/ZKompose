use sha2::{Digest, Sha256};

pub fn hash_f64(hash_u: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(1_000.0);
    let range = max - min;
    let mut hasher = Sha256::new();
    hasher.update(hash_u.to_be_bytes());
    let res = min
        + hasher
            .finalize()
            .as_slice()
            .iter()
            .fold(0.0, |acc, &x| acc + x as f64)
            % range;

    res / range
}

pub fn new_u_v(u: f64, v: f64) -> (f64, f64) {
    let u = hash_f64(u, None, None);
    let v = hash_f64(v, None, None);

    (u, v)
}

pub fn hash_i32(hash_u: f64, min: Option<i32>, max: Option<i32>) -> i32 {
    let min = min.unwrap_or(0);
    let max = max.unwrap_or(100_000);
    let mut hasher = Sha256::new();
    hasher.update(hash_u.to_be_bytes());
    min + hasher
        .finalize()
        .as_slice()
        .iter()
        .fold(0, |acc, &x| acc + x as i32)
        % (max - min)
}

fn normal_random(u: f64, v: f64) -> f64 {
    //   return Math.sqrt(-2.0 * Math.log(u)) * Math.cos(2.0 * Math.PI * v);

    (-2.0 * u.ln()).sqrt() * (2.0 * std::f64::consts::PI * v).cos()
}

pub fn generate_max_rating(std_dev: f64, median: f64, u: f64, v: f64) -> u8 {
    let normal = normal_random(u, v);

    println!("u, v: {}, {}", u, v);
    println!("Normal: {}, {}, {}", normal, std_dev, median);

    let rating = (normal * std_dev + median).round() as i32;
    rating.max(40).min(99) as u8
}

pub fn generate_skill_scores(overall_rating: u8, u: f64, v: f64) -> Vec<u8> {
    let base_skill = (overall_rating as i32 - 20).max(1) as u8;
    let skill_spread = ((overall_rating - base_skill) / 2) as u8;
    let mut u = u;
    let mut v = v;

    let mut skills: Vec<u8> = (0..7)
        .map(|_| {
            // let skill = base_skill as i32 + rng.gen_range(0..=skill_spread as i32 * 2);
            (u, v) = new_u_v(u, v);
            let skill = hash_i32(u + v, Some(0), Some(skill_spread as i32));
            let skill = base_skill as i32 + skill;

            skill.max(1).min(99) as u8
        })
        .collect();

    // let primary_skill_index = rng.gen_range(0..skills.len());
    let (u, v) = new_u_v(u, v);
    let primary_skill_index = hash_i32(u + v, Some(0), Some(skills.len() as i32)) as usize;

    skills[primary_skill_index] = overall_rating - hash_i32(u + v, Some(0), Some(5)) as u8;

    skills
}

pub fn calculate_tier(overall_rating: u8) -> u8 {
    match overall_rating {
        90..=99 => 0, // Superstar
        80..=89 => 1, // All-Star
        70..=79 => 2, // Starter
        60..=69 => 3, // Rotation
        _ => 4,       // Bench
    }
}
