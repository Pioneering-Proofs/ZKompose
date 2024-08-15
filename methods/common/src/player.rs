use super::math::{calculate_tier, generate_max_rating, generate_skill_scores, hash_i32};
use super::types::{
    Attribute, CIDError, ContentAddressable, FileStats, Player, PlayerJson, Skills,
};
use super::utils::compute_cid;
use cid::Cid;
use std::env;

impl Player {
    pub fn cid(&self) -> [u8; 34] {
        compute_cid(self.fill_template().as_bytes())
            .cid
            .try_into()
            .expect("Failed to extract CID bytes")
    }

    pub fn cid_string(&self) -> Result<String, CIDError> {
        let cid_bytes = self.cid();
        if cid_bytes.is_empty() {
            return Err(CIDError::EmptyCID);
        }
        let cid = Cid::try_from(cid_bytes.to_vec());
        match cid {
            Ok(cid) => Ok(cid.to_string()),
            Err(_) => Err(CIDError::DecodeFailed),
        }
    }

    pub fn formatted_cid(&self) -> Result<String, CIDError> {
        let cid_string = self.cid_string()?;
        Ok(["ipfs://", &cid_string].concat())
    }

    // TODO: conform to Template trait
    pub fn fill_template(&self) -> String {
        let base_uri = env::var("PLAYER_EXTERNAL_URL_BASE")
            .unwrap_or("https://www.youtube.com/watch?v=dQw4w9WgXcQ?".to_string());

        let template_str = self
            .template()
            .replace("NAME", &self.name())
            .replace("JERSEY_NUMBER", &self.jersey_number.to_string())
            .replace("EXTERNAL_URL", &format!("{}{}", base_uri, self.token_id))
            .replace(
                "IMAGE_URI",
                format!("ipfs://{}", "TODO: IPFS SVG HASH").as_str(),
            )
            .replace("TIER", self.tier().to_string().as_str())
            .replace("OVERALL_RATING", self.overall_rating.to_string().as_str())
            .replace("SKILL_MULTIPLIER", &self.skill_multiplier().to_string())
            .replace("SPEED", &self.skills.speed.to_string())
            .replace("SHOOTING", &self.skills.shooting.to_string())
            .replace("PASSING", &self.skills.passing.to_string())
            .replace("DRIBBLING", &self.skills.dribbling.to_string())
            .replace("DEFENSE", &self.skills.defense.to_string())
            .replace("PHYSICAL", &self.skills.physical.to_string())
            .replace("GOAL_TENDING", &self.skills.goal_tending.to_string());

        template_str
    }

    fn template(&self) -> &str {
        let template = include_str!("../../../templates/player/metadata.json");
        &template
    }
}

impl Player {
    pub fn new(token_id: u32, standard_deviation: u8, median: u8, u: f64, v: f64) -> Self {
        let max_rating = generate_max_rating(standard_deviation as f64, median as f64, u, v);

        println!("Max rating: {}", max_rating);

        let skill_scores = generate_skill_scores(max_rating, u, v);
        let jersey_number: u8 = hash_i32(u + v, Some(0), Some(999)) as u8;
        let name_indices = [
            hash_i32(u + v, Some(0), Some(24)) as u8,
            hash_i32(u + v, Some(0), Some(24)) as u8,
        ];

        let skills = Skills {
            speed: skill_scores[0],
            shooting: skill_scores[1],
            passing: skill_scores[2],
            dribbling: skill_scores[3],
            defense: skill_scores[4],
            physical: skill_scores[5],
            goal_tending: skill_scores[6],
        };
        let overall_rating = skill_scores.iter().max().unwrap().clone();

        Player {
            token_id,
            name_indicies: name_indices,
            overall_rating,
            skills,
            skill_multiplier_bips: 10_000,
            jersey_number,
        }
    }

    pub fn skill_multiplier(&self) -> f32 {
        self.skill_multiplier_bips as f32 / 10_000.0
    }

    pub fn name(&self) -> String {
        make_name(self.name_indicies[0] as i32, self.name_indicies[1] as i32)
    }

    pub(crate) fn name_indices(name: String) -> Result<[u8; 2], &'static str> {
        let mut name_indices = [0; 2];
        let name_parts: Vec<&str> = name.split(" ").collect();
        if name_parts.len() == 2 {
            name_indices[0] = FIRST_NAMES
                .iter()
                .position(|&r| r == name_parts[0])
                .expect("Invalid first name") as u8;
            name_indices[1] = LAST_NAMES
                .iter()
                .position(|&r| r == name_parts[1])
                .expect("Invalid last name") as u8;
        } else {
            return Err("Invalid name. Must be 2 words longs.");
        }
        Ok(name_indices)
    }

    pub fn tier(&self) -> u8 {
        calculate_tier(self.overall_rating)
    }

    pub fn attributes(&self) -> Vec<Attribute> {
        let mut attributes = Vec::new();
        attributes.push(Attribute {
            trait_type: "Speed".to_string(),
            display_type: "Speed".to_string(),
            value: self.skills.speed,
        });
        attributes.push(Attribute {
            trait_type: "Shooting".to_string(),
            display_type: "Shooting".to_string(),
            value: self.skills.shooting,
        });
        attributes.push(Attribute {
            trait_type: "Passing".to_string(),
            display_type: "Passing".to_string(),
            value: self.skills.passing,
        });
        attributes.push(Attribute {
            trait_type: "Dribbling".to_string(),
            display_type: "Dribbling".to_string(),
            value: self.skills.dribbling,
        });
        attributes.push(Attribute {
            trait_type: "Defense".to_string(),
            display_type: "Defense".to_string(),
            value: self.skills.defense,
        });
        attributes.push(Attribute {
            trait_type: "Physical".to_string(),
            display_type: "Physical".to_string(),
            value: self.skills.physical,
        });
        attributes.push(Attribute {
            trait_type: "Goal Tending".to_string(),
            display_type: "Goal Tending".to_string(),
            value: self.skills.goal_tending,
        });

        attributes
    }

    pub fn to_json(&self) -> PlayerJson {
        PlayerJson {
            name: self.name().clone(),
            overall_rating: self.overall_rating,
            skill: self.skills.clone(),
            skill_multiplier: self.skill_multiplier(),
            jersey_number: self.jersey_number,
            description: format!(
                "#{} {}. Overall rating of {}.",
                self.jersey_number,
                self.name(),
                self.overall_rating
            ),
            external_url: "https://TODO: NEXTJS URL".to_string(),
            image: format!(
                "ipfs://{}/{}.svg",
                Player::player_svg_dir_cid(),
                self.jersey_number,
            ),
            tier: self.tier(),
            attributes: self.attributes(),
        }
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self.to_json()).unwrap()
    }

    pub fn player_svg_dir_cid() -> String {
        "QmXg8bcnf2HpEaedTwek2eHsiEGCrZh8fYkLtGzsoeCqRk".to_string()
    }
}

// impl PlayerMetadata {
//     pub fn name(&self) -> String {
//         match &self.name {
//             Some(name) => name.clone(),
//             None => random_name(self.jersey_number as i32),
//         }
//     }

//     pub fn description(&self) -> String {
//         format!(
//             "#{} {}. Overall rating of {}.",
//             self.jersey_number,
//             self.name(),
//             self.overall_rating
//         )
//     }
// }

impl PlayerJson {
    pub fn random_name(seed: i32) -> String {
        random_name(seed)
    }
}

const FIRST_NAMES: [&str; 25] = [
    "James", "John", "Robert", "Michael", "William", "David", "Richard", "Joseph", "Thomas",
    "Charles", "Daniel", "Matthew", "Anthony", "Donald", "Mark", "Paul", "Steven", "Andrew",
    "Kenneth", "Joshua", "George", "Kevin", "Brian", "Edward", "Ronald",
];
const LAST_NAMES: [&str; 25] = [
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

fn make_name(first_name_index: i32, last_name_index: i32) -> String {
    format!(
        "{} {}",
        FIRST_NAMES[first_name_index as usize], LAST_NAMES[last_name_index as usize]
    )
}

fn random_name(seed: i32) -> String {
    let first_name_index = seed % FIRST_NAMES.len() as i32;
    let last_name_index = seed % LAST_NAMES.len() as i32;

    make_name(first_name_index, last_name_index)
}
