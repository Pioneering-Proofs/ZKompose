use super::types::{Coach, Player, Roster, Skills, Team};
use array_init::try_array_init;
use json::JsonValue;
use std::{
    collections::HashSet,
    convert::TryFrom,
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug)]
pub enum DecodingError {
    InvalidTeamSize,
    ReusedPlayer(Player),
    InvalidCoach,
}

impl TryFrom<JsonValue> for Team {
    type Error = DecodingError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        Ok(Team {
            name: String::from(value["name"].as_str().expect("No team name")),
            logo: match value.contains("logo") {
                true => Some(String::from(value["logo"].as_str().unwrap())),
                false => None,
            },
            coach: Coach::try_from(value["coach"].clone()).unwrap(),
            roster: Roster::try_from(value["roster"].clone()).unwrap(),
        })
    }
}

impl TryFrom<JsonValue> for Player {
    type Error = DecodingError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        Ok(Player {
            token_id: value["token_id"].as_u32().expect("No token_id"),
            cid: String::from(value["cid"].as_str().expect("No CID")),
            skills: Skills::try_from(value["skills"].clone()).unwrap(),
            skill_multiplier: value["skill_multiplier"]
                .as_f32()
                .expect("No skill_multiplier"),
        })
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
            goal_tending: value["goalTending"].as_u8().expect("No goalTending"),
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
        let defenders = try_array_init(|n| check_player(n, "defenders")).unwrap();
        let midfielders = try_array_init(|n| check_player(n, "midfielders")).unwrap();
        let forwards = try_array_init(|n| check_player(n, "forwards")).unwrap();

        Ok(Roster {
            goal_keeper: Player::try_from(value["goal_keeper"].clone()).unwrap(),
            defenders,
            midfielders,
            forwards,
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
