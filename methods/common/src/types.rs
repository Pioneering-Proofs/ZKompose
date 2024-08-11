use serde::{Deserialize, Serialize};
// TODO: Cid does not conform to Serialize. Using Str for now
// use cid::Cid;

///
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Team {
    pub roster: Roster,
    pub coach: Coach,
    pub name: String,
    pub logo: Option<String>, //Cid,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Roster {
    pub goal_keeper: Player,
    pub defenders: [Player; 4],
    pub midfielders: [Player; 3],
    pub forwards: [Player; 3],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Coach {
    pub name: String,
    pub goal_muliplier: f32,
    pub defense_multiplier: f32,
    pub midfield_multiplier: f32,
    pub forward_multiplier: f32,
}

/// Player struct encodes the on-chain token ID, the CID of the player's IPFS data, the player's traits, and a skill multiplier.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Player {
    pub token_id: u32,
    pub cid: String, //Cid,
    pub skills: Skills,
    pub skill_multiplier: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PlayerPosition {
    Goalie(Player),
    Defense(Player, u8),
    Mid(Player, u8),
    Offense(Player, u8),
}

/// Traits encodes an individual player's performance statistics.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Skills {
    pub speed: u8,
    pub shooting: u8,
    pub passing: u8,
    pub dribbling: u8,
    pub defense: u8,
    pub physical: u8,
    pub goal_tending: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenPlayersInput {
    pub buyer_pubkey: String,
    pub std_dev: u8,
    pub median: u8,
    pub u: f64,
    pub v: f64,
}

/// Player data contains the usable Player struct as well as the raw bytes of the player's IPFS data which is used to validate the CID.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlayerData {
    pub player: PlayerPosition,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlayerJson {
    pub name: String,
    pub description: String,
    pub external_url: String,
    pub image: String,
    pub jersey_number: u8,
    pub tier: u8,
    pub overall_rating: u8,
    pub attributes: Vec<Attribute>,
    pub skill_multiplier: f32,
    pub skill: Skill,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Attribute {
    pub display_type: String,
    pub trait_type: String,
    pub value: u8,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Skill {
    pub speed: u8,
    pub shooting: u8,
    pub passing: u8,
    pub dribbling: u8,
    pub defense: u8,
    pub physical: u8,
    pub goal_tending: u8,
}
