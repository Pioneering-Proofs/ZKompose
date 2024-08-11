use super::types::{
    CIDError, Coach, ContentAddressable, FileStats, Player, PlayerData, PlayerJson, Roster, Skills,
    Team,
};
use super::utils::compute_cid;

impl ContentAddressable for Player {
    fn content_stats(&self) -> FileStats {
        let mut stats = FileStats::default();
        stats.cid = self.cid.clone();
        // TODO: Need to fill in the template file
        stats
    }
}

impl Player {
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
}

impl PlayerJson {
    pub fn random_name(seed: i32) -> String {
        let first_name: [&str; 25] = [
            "James", "John", "Robert", "Michael", "William", "David", "Richard", "Joseph",
            "Thomas", "Charles", "Daniel", "Matthew", "Anthony", "Donald", "Mark", "Paul",
            "Steven", "Andrew", "Kenneth", "Joshua", "George", "Kevin", "Brian", "Edward",
            "Ronald",
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
}
