use alloy_primitives::{address, Address, U256};
use risc0_steel::ethereum::EthEvmInput;
use serde::{Deserialize, Serialize};

pub(crate) trait ContentAddressable {
    fn content_stats(&self) -> FileStats;
}

/// Template trait is used by types who have associated metadata or other template files which get filled with data.
pub trait Template {
    fn fill_template(&self) -> &str;
    fn template(&self) -> &str;
}

#[derive(Clone, Debug)]
pub enum CIDError {
    EmptyCID,
    NoDataBytes,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FileStats {
    pub cid: Option<String>, //Cid,
    pub blocks: usize,
    pub bytes: u64,
}

///
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Team {
    pub roster: Roster,
    // pub coach: Coach,
    pub name: String,
    pub logo: Option<String>, //Cid,
    pub team_rating: u8,
}

/// Player struct encodes the on-chain token ID, the CID of the player's IPFS data, the player's traits, and a skill multiplier.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Player {
    pub token_id: u32,
    pub cid: Option<String>, //Cid,
    pub name: String,
    pub overall_rating: u8,
    pub skills: Skills,
    pub skill_multiplier: f32,
    pub jersey_number: u8,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlayerCreationParams {
    pub(crate) standard_deviation: u8,
    pub(crate) median: u8,
    pub(crate) u: f64,
    pub(crate) v: f64,
}

// #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
// pub struct PlayerMetadata {
//     pub(crate) name: Option<String>,
//     pub(crate) external_url: String,
//     pub(crate) image: String,
//     pub jersey_number: u8,
// }

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
    pub order_id: u32,
    pub std_dev: u8,
    pub median: u8,
    pub u: f64,
    pub v: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenTeamInput {
    pub name: String,
    pub roster: Roster,
    pub owner: Address,
    pub logo_uri: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuildTeamInput {
    pub chain_config: EthEvmInput,
    pub owner: Address,
    pub roster: Roster,
}

// TODO: Drop this in lieu of PlayerMetadata
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlayerJson {
    pub name: String,
    pub description: String,
    pub external_url: String,
    pub image: String,
    pub jersey_number: u8,
    pub tier: u8, //
    pub overall_rating: u8,
    pub attributes: Vec<Attribute>,
    pub skill_multiplier: f32,
    pub skill: Skills,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Roster {
    pub goal_tender: Player,
    pub defense: [Player; 4],
    pub mid: [Player; 3],
    pub offense: [Player; 3],
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Coach {
    pub name: String,
    pub goal_muliplier: f32,
    pub defense_multiplier: f32,
    pub midfield_multiplier: f32,
    pub forward_multiplier: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PlayerPosition {
    Goalie(Player),
    Defense(Player, u8),
    Mid(Player, u8),
    Offense(Player, u8),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Attribute {
    pub display_type: String,
    pub trait_type: String,
    pub value: u8,
}
