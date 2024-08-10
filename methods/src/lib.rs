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
    use alloy_primitives::{address, Address, U256, U8};
    use alloy_sol_types::SolValue;
    use common::types::{GenPlayersInput, Player, PlayerData, PlayerPosition, Skills, Team};
    use json::{parse, stringify};
    use risc0_zkvm::{default_executor, guest::env::write_slice, serde, ExecutorEnv};
    use std::{env::current_dir, fs};

    #[test]
    // fn prove_build_team() {
    //     let input_data = include_str!("../../data/teams/0.json");
    //     // println!("Input data: {}", input_data);
    //     let mut input_data = parse(input_data).unwrap();
    //     for n in 0..10 {
    //         let current = current_dir().unwrap();
    //         let file_name: String;
    //         if current.ends_with("methods") {
    //             file_name = format!("../data/players/{}.json", n);
    //         } else {
    //             file_name = format!("../../data/players/{}.json", n);
    //         }
    //         println!(
    //             "Reading player data from: {} from {}",
    //             file_name,
    //             current.display()
    //         );
    //         let player_data =
    //             fs::read_to_string(file_name).expect("Should have been able to read the file");
    //         input_data["players"][n] = parse(&player_data).unwrap();
    //     }
    //     println!("Running build team. Data length: {}", input_data.len());
    //     // println!("Input data: {}", input_data.to_string());

    //     let env = ExecutorEnv::builder()
    //         .write(&input_data.to_string())
    //         .unwrap()
    //         .build()
    //         .unwrap();

    //     let session_info = default_executor()
    //         .execute(env, super::BUILD_TEAM_ELF)
    //         .unwrap();
    // }
    #[test]
    fn prove_gen_players() {
        // let player_count = U8::from(10);
        // let std_dev = U8::from(10);
        // let median = U8::from(50);
        let input = GenPlayersInput {
            player_count: 10,
            std_dev: 10,
            median: 50,
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

        let players: Vec<PlayerData> = serde::from_slice(&session_info.journal.bytes).unwrap();

        println!("Session info: {:?}", players);
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
