use super::types::{Coach, FileStats, Player, Roster, Skills, Team};
use array_init::try_array_init;
use ipfs_unixfs::{dir::builder::BufferingTreeBuilder, file::adder::FileAdder};
use json::JsonValue;
use std::{
    collections::HashSet,
    convert::TryFrom,
    hash::{Hash, Hasher},
    io::Read,
};

////////////////////////////////////////////////////////////////////////
/// IPFS Utils
////////////////////////////////////////////////////////////////////////

pub fn compute_cid(input: &[u8]) -> FileStats {
    let mut adder = FileAdder::default();

    let length = input.len();

    for i in 0..length {
        adder.push(&[input[i]]);
    }

    let blocks = adder.finish();
    let mut stats = FileStats::default();
    for (cid, block) in blocks {
        // [18, 32, 220, 232, 214, 52, 213, 200, 177, 163, 3, 84, 141, 30, 88, 42, 34, 45, 50, 100, 19, 248, 7, 94, 22, 62, 7, 10, 14, 149, 143, 160, 50, 5]
        stats.cid = Some(cid.to_string());
        stats.blocks += 1;
        stats.bytes += block.len() as u64;
    }

    stats
}

// pub fn compute_hash(input: &[u8]) -> [u8; 32] {
//     let mut adder = FileAdder::default();

//     let length = input.len();

//     for i in 0..length {
//         adder.push(&[input[i]]);
//     }

//     let blocks = adder.finish();
//     let mut stats = FileStats::default();
//     let mut result;
//     for (cid, block) in blocks {
//         // [18, 32, 220, 232, 214, 52, 213, 200, 177, 163, 3, 84, 141, 30, 88, 42, 34, 45, 50, 100, 19, 248, 7, 94, 22, 62, 7, 10, 14, 149, 143, 160, 50, 5]
//         result = cid;
//         stats.cid = Some(cid.to_string());
//         stats.blocks += 1;
//         stats.bytes += block.len() as u64;
//     }

//     result.hash().bytes().try_into().unwrap()
// }

////////////////////////////////////////////////////////////////////////
/// Serialization and Deserialization
////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub enum DecodingError {
    InvalidTeamSize,
    ReusedPlayer(Player),
    InvalidCoach,
    MissingJsonField(&'static str),
    UnknownError,
}

impl TryFrom<JsonValue> for Team {
    type Error = DecodingError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        Ok(Team::new(
            Roster::try_from(value["roster"].clone()).unwrap(),
            String::from(value["name"].as_str().expect("No team name")),
            match value.contains("logo") {
                true => Some(String::from(value["logo"].as_str().unwrap())),
                false => None,
            },
        ))
    }
}

/// Allows Player struct to be directly created from JSON String
impl TryFrom<String> for Player {
    type Error = DecodingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let json_parsed = json::parse(&value).unwrap();
        match Player::try_from(json_parsed) {
            Ok(mut player) => {
                player.compute_cid(value.clone().to_string().as_bytes());
                Ok(player)
            }
            Err(e) => Err(e),
        }
    }
}

impl TryFrom<JsonValue> for Player {
    type Error = DecodingError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        let token_id = value["token_id"].as_u32();
        let cid = match value["cid"].as_str() {
            Some(cid) => Some(String::from(cid)),
            None => None,
        };
        let name = value["name"].as_str();
        let overall_rating = value["overall_rating"].as_u8();
        let jersey_number = value["jersey_number"].as_u8();
        let skills = Skills::try_from(value["skill"].clone());
        let skill_multiplier = value["skill_multiplier"].as_f32();

        // Try decoding as simple json
        if token_id.is_some()
            && skills.is_ok()
            && skill_multiplier.is_some()
            && name.is_some()
            && overall_rating.is_some()
            && jersey_number.is_some()
        {
            Ok(Player {
                token_id: token_id.unwrap(),
                cid,
                name: String::from(name.unwrap()),
                overall_rating: overall_rating.unwrap(),
                jersey_number: jersey_number.unwrap(),
                skills: skills.unwrap(),
                skill_multiplier: skill_multiplier.unwrap(),
            })
        } else {
            // if simple decoding fails, try parsing as NFT metadata
            // TODO: Try parsing in other format
            if token_id.is_none() {
                Err(DecodingError::MissingJsonField("token_id"))
            } else if skills.is_err() {
                Err(DecodingError::MissingJsonField("skills"))
            } else if skill_multiplier.is_none() {
                Err(DecodingError::MissingJsonField("skill_multiplier"))
            } else {
                Err(DecodingError::UnknownError)
            }
        }
    }
}

impl TryFrom<JsonValue> for Skills {
    type Error = DecodingError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        Ok(Skills {
            speed: value["speed"].as_u8().expect("No speed"),
            shooting: value["shooting"].as_u8().expect("No shooting"),
            passing: value["passing"].as_u8().expect("No passing"),
            dribbling: value["dribbling"].as_u8().expect("No dribbling"),
            defense: value["defense"].as_u8().expect("No defense"),
            physical: value["physical"].as_u8().expect("No physical"),
            goal_tending: value["goal_tending"].as_u8().expect("No goal_tending"),
        })
    }
}

impl TryFrom<JsonValue> for Roster {
    type Error = DecodingError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        let players: HashSet<Player> = HashSet::new();
        let check_player = |n: usize, key: &str| {
            let player = Player::try_from(value[key][n].clone()).unwrap();
            if players.contains(&player) {
                Err(DecodingError::ReusedPlayer(player))
            } else {
                Ok(player)
            }
        };
        let defense = try_array_init(|n| check_player(n, "defense")).unwrap();
        let mid = try_array_init(|n| check_player(n, "mid")).unwrap();
        let offense = try_array_init(|n| check_player(n, "offense")).unwrap();

        Ok(Roster {
            goal_tender: Player::try_from(value["goal_tender"].clone()).unwrap(),
            defense,
            mid,
            offense,
        })
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token_id.hash(state);
        self.cid.hash(state);
        self.skills.hash(state);
    }
}

impl Eq for Player {}

impl Hash for Skills {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.speed.hash(state);
        self.shooting.hash(state);
        self.passing.hash(state);
        self.dribbling.hash(state);
        self.defense.hash(state);
        self.physical.hash(state);
        self.goal_tending.hash(state);
    }
}

impl TryFrom<JsonValue> for Coach {
    type Error = DecodingError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        let result = Coach {
            name: String::from(value["name"].as_str().expect("No coach name")),
            goal_muliplier: value["goal_muliplier"].as_f32().expect("No goal_muliplier"),
            defense_multiplier: value["defense_multiplier"]
                .as_f32()
                .expect("No defense_multiplier"),
            midfield_multiplier: value["midfield_multiplier"]
                .as_f32()
                .expect("No midfield_multiplier"),
            forward_multiplier: value["forward_multiplier"]
                .as_f32()
                .expect("No forward_multiplier"),
        };
        if result.goal_muliplier != 1_f32
            || result.defense_multiplier != 1_f32
            || result.midfield_multiplier != 1_f32
            || result.forward_multiplier != 1_f32
        {
            Err(DecodingError::InvalidCoach)
        } else {
            Ok(result)
        }
    }
}
