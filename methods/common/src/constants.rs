use alloy_primitives::{address, Address};

pub const SHA256_PREFIX: [u8; 2] = [18_u8, 32_u8];

pub const PLAYER_STD_DEV: u8 = 10;
pub const PLAYER_BATCH_SIZE: usize = 15;

pub const PLAYER_CONTRACT_ADDRESS: Address = address!("a22c4fAAa1f7e4CAFE3F40D602A815c3B688b123");
pub const TEAM_CONTRACT_ADDRESS: Address = address!("cD9ec6698106f294D1eC51Cc72A4440FBad79127");
