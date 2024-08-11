use super::math::{calculate_tier, generate_max_rating, generate_skill_scores, hash_f64, hash_i32};
use super::types::{
    Attribute, CIDError, Coach, ContentAddressable, FileStats, Player, PlayerCreationParams,
    PlayerJson, Roster, Skills, Team, Template,
};
use super::utils::compute_cid;
use std::env;
use std::fmt::format;

impl ContentAddressable for Player {
    fn content_stats(&self) -> FileStats {
        let mut stats = FileStats::default();
        stats.cid = self.cid.clone();
        // TODO: Need to fill in the template file
        stats
    }
}

impl Player {
    pub fn cid(&self) -> String {
        compute_cid(self.fill_template().as_bytes()).cid.unwrap()
    }

    // TODO: conform to Template trait
    pub fn fill_template(&self) -> String {
        let base_uri = env::var("PLAYER_EXTERNAL_URL_BASE")
            .unwrap_or("https://www.youtube.com/watch?v=dQw4w9WgXcQ?".to_string());

        let template_str = self
            .template()
            .replace("NAME", &self.name)
            .replace("JERSEY_NUMBER", &self.jersey_number.to_string())
            .replace("EXTERNAL_URL", &format!("{}{}", base_uri, self.token_id))
            .replace(
                "IMAGE_URI",
                format!("ipfs://{}", "TODO: IPFS SVG HASH").as_str(),
            )
            .replace("TIER", self.tier().to_string().as_str())
            .replace("OVERALL_RATING", self.overall_rating.to_string().as_str())
            .replace("SKILL_MULTIPLIER", &self.skill_multiplier.to_string())
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
            cid: None,
            name: random_name(hash_i32(u + v, Some(0), Some(24))),
            overall_rating,
            skills,
            skill_multiplier: 1.0,
            jersey_number,
        }
    }

    pub fn matches_cid(&self, input: &[u8]) -> Result<bool, CIDError> {
        let cid = match self.cid {
            Some(ref cid) => cid,
            None => return Err(CIDError::EmptyCID),
        };

        let stats = compute_cid(input);
        match stats.cid {
            Some(ref c) => Ok(c == cid),
            None => Err(CIDError::NoDataBytes),
        }
    }

    pub fn compute_cid(&mut self, input: &[u8]) {
        let stats = compute_cid(input);
        self.cid = stats.cid;
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
            name: self.name.clone(),
            overall_rating: self.overall_rating,
            skill: self.skills.clone(),
            skill_multiplier: self.skill_multiplier,
            jersey_number: self.jersey_number,
            description: format!(
                "#{} {}. Overall rating of {}.",
                self.jersey_number, self.name, self.overall_rating
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
