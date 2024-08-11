// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Generated crate containing the image ID and ELF binary of the build guest.
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(test)]
mod tests {
    use alloy_primitives::{address, U256};
    use alloy_sol_types::SolValue;
    use common::types::{
        GenPlayersInput, GenPlayersJournal, GenTeamInput, Player, PlayerJson, Roster, Skills, Team,
    };
    use json::{parse, stringify};
    use risc0_zkvm::{default_executor, guest::env::write_slice, serde, ExecutorEnv};
    use std::{env::current_dir, fs};

    #[test]
    fn prove_build_team() {
        let mut players: Vec<Player> = vec![];
        for n in 0..=10 {
            let path = format!("../../test_data/players/{}.json", n);
            let input_data = fs::read_to_string(path).unwrap();
            let input_data = parse(&input_data).unwrap();
            let player = Player::try_from(input_data.clone()).unwrap();
            players.push(player);
        }
        let input = GenTeamInput {
            roster: Roster {
                goal_tender: players[0].clone(),
                defense: [
                    players[1].clone(),
                    players[2].clone(),
                    players[3].clone(),
                    players[4].clone(),
                ],
                mid: [players[5].clone(), players[6].clone(), players[7].clone()],
                offense: [players[8].clone(), players[9].clone(), players[10].clone()],
            },
            name: "Test Team".to_string(),
            owner: address!("d8da6bf26964af9d7eed9e03e53415d37aa96045"),
            logo_uri: None,
        };

        let env = ExecutorEnv::builder()
            .write(&input)
            .unwrap()
            .build()
            .unwrap();

        let session_info = default_executor()
            .execute(env, super::BUILD_TEAM_ELF)
            .unwrap();

        let cids: [String; 15] = serde::from_slice(&session_info.journal.bytes)
            .expect("Failed to decode players from guest");
    }

    #[test]
    fn player_cid() {
        let current = current_dir().unwrap();
        let file_name: String;
        if current.ends_with("methods") {
            file_name = "../data/players/0.json".to_string();
        } else {
            file_name = "../../data/players/0.json".to_string();
        }
        let player_data =
            fs::read_to_string(file_name).expect("Should have been able to read the file");
        let player = Player::try_from(player_data).unwrap();
        println!("Player data: {:?}", player);
    }

    #[test]
    fn prove_gen_players() {
        let input = GenPlayersInput {
            buyer_pubkey: "".to_string(),
            order_id: 42,
            std_dev: 10,
            tier: 1,
            u: 3.14159,
            v: 2123.71828,
        };

        let env = ExecutorEnv::builder()
            .write(&input)
            .expect("Invalid input")
            .build()
            .unwrap();

        let session_info = default_executor()
            .execute(env, super::GEN_PLAYER_ELF)
            .unwrap();

        println!("Generated players: {:?}", session_info.journal.bytes);

        let output: GenPlayersJournal = serde::from_slice(&session_info.journal.bytes)
            .expect("Failed to decode players from guest");

        println!("Player data: {:?}", output.cids.len());
        println!("Tier: {}", output.tier);
        println!("Order ID: {}", output.order_id);

        for cid in output.cids.iter() {
            println!("CID: {:?}", cid);
        }
    }

    #[test]
    fn proves_even_number() {
        let even_number = U256::from(1304);

        let env = ExecutorEnv::builder()
            .write_slice(&even_number.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        let session_info = default_executor().execute(env, super::IS_EVEN_ELF).unwrap();

        let x = U256::abi_decode(&session_info.journal.bytes, true).unwrap();
        assert_eq!(x, even_number);
    }

    #[test]
    #[should_panic(expected = "number is not even")]
    fn rejects_odd_number() {
        let odd_number = U256::from(75);

        let env = ExecutorEnv::builder()
            .write_slice(&odd_number.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        default_executor().execute(env, super::IS_EVEN_ELF).unwrap();
    }
}
