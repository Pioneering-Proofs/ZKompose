use super::types::{Coach, Player, PlayerData, PlayerJson, Roster, Skills, Team};
use ipfs_unixfs::file::adder::FileAdder;

impl Player {
    fn matches_cid(&self, input: &[u8]) -> bool {
        let mut adder = FileAdder::default();

        true
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
