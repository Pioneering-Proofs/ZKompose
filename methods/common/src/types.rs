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

/// Player data contains the usable Player struct as well as the raw bytes of the player's IPFS data which is used to validate the CID.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlayerData {
    pub player: PlayerPosition,
    pub bytes: Vec<u8>,
}
