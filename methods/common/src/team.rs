use crate::utils::compute_cid;

use super::types::{PlayerPosition, Roster, Team};
use serde_json;

impl IntoIterator for Roster {
    type Item = PlayerPosition;
    type IntoIter = RosterIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        RosterIntoIterator {
            roster: self,
            index: 0,
        }
    }
}

pub struct RosterIntoIterator {
    roster: Roster,
    index: usize,
}

impl Iterator for RosterIntoIterator {
    type Item = PlayerPosition;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(PlayerPosition::Goalie(self.roster.goal_tender.clone())),
            1..=4 => Some(PlayerPosition::Defense(
                self.roster.defense[self.index - 1].clone(),
                self.index as u8 - 1 as u8,
            )),
            5..=7 => Some(PlayerPosition::Mid(
                self.roster.mid[self.index - 1].clone(),
                self.index as u8 - 4 as u8,
            )),
            8..=10 => Some(PlayerPosition::Offense(
                self.roster.offense[self.index - 1].clone(),
                self.index as u8 - 7 as u8,
            )),
            _ => return None,
        };
        self.index += 1;
        result
    }
}

impl Team {
    pub fn new(roster: Roster, name: String, logo: Option<String>) -> Self {
        Self {
            roster: roster.clone(),
            name,
            logo,
            team_rating: Self::team_rating(roster),
        }
    }

    pub fn team_rating(roster: Roster) -> u8 {
        let mut rating = 0;
        let players = vec![
            roster.goal_tender.clone(),
            roster.defense[0].clone(),
            roster.defense[1].clone(),
            roster.defense[2].clone(),
            roster.defense[3].clone(),
            roster.mid[0].clone(),
            roster.mid[1].clone(),
            roster.mid[2].clone(),
            roster.offense[0].clone(),
            roster.offense[1].clone(),
            roster.offense[2].clone(),
        ];

        for player in players {
            rating += player.overall_rating;
        }

        rating / 11
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn cid(&self) -> String {
        compute_cid(self.to_json().as_bytes()).cid.unwrap()
    }
}
