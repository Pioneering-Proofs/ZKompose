use super::types::{Player, PlayerPosition, Roster, Team};

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
            0 => Some(PlayerPosition::Goalie(self.roster.goal_keeper.clone())),
            1..=4 => Some(PlayerPosition::Defense(
                self.roster.defenders[self.index - 1].clone(),
                self.index as u8 - 1 as u8,
            )),
            5..=7 => Some(PlayerPosition::Mid(
                self.roster.defenders[self.index - 1].clone(),
                self.index as u8 - 4 as u8,
            )),
            8..=10 => Some(PlayerPosition::Offense(
                self.roster.defenders[self.index - 1].clone(),
                self.index as u8 - 7 as u8,
            )),
            _ => return None,
        };
        self.index += 1;
        result
    }
}
