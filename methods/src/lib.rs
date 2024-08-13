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
    use alloy_primitives::{address, hex::ToHex, U256};
    use alloy_sol_types::{sol, SolValue};
    use cid::{multihash, Cid};
    use common::types::{
        GenPlayersInput, GenPlayersJournal, GenTeamInput, Player, PlayerJson, Roster, Skills, Team,
    };
    use json::{parse, stringify};
    use risc0_zkvm::{
        default_executor, default_prover, guest::env::write_slice, serde, ExecutorEnv, ProverOpts,
        VerifierContext,
    };
    use std::{env::current_dir, fs, io::Read};

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
        sol! {
            struct Input {
                uint8 tier;
                uint256 order_id;
                uint8 std_dev;
                uint256 u;
                uint256 v;
            }

            struct Journal {
                uint8 tier;
                uint256 order_id;
                bytes32[15] cids;
            }
        }
        let input = Input {
            order_id: U256::from(42),
            std_dev: 10,
            tier: 1,
            u: U256::from(3.14159),
            v: U256::from(2123.71828),
        }
        .abi_encode();

        let env = ExecutorEnv::builder().write_slice(&input).build().unwrap();

        let session_info = default_executor()
            .execute(env, super::GEN_PLAYER_ELF)
            .unwrap();

        println!("Generated players: {:?}", session_info.journal.bytes);

        let output: Journal = Journal::abi_decode(&session_info.journal.bytes, true)
            .expect("Failed to decode output journal");

        println!("Player data: {:?}", output.cids.len());
        println!("Tier: {}", output.tier);
        println!("Order ID: {}", output.order_id);

        for cid in output.cids.iter() {
            let prefix = vec![0x12_u8, 0x20_u8];
            let hash = Vec::<u8>::from(cid.0);
            let multihash = multihash::Multihash::from_bytes([prefix, hash].concat().as_slice())
                .expect("Failed to create multihash");
            let cid = Cid::new_v0(multihash).unwrap();
            println!("CID: {:?}", cid);
        }

        // let env = ExecutorEnv::builder()
        //     .write(&input)
        //     .expect("Invalid input")
        //     .build()
        //     .unwrap();

        // let prover = default_prover();
        // let receipt = prover
        //     .prove_with_ctx(
        //         env,
        //         &VerifierContext::default(),
        //         super::GEN_PLAYER_ELF,
        //         &ProverOpts::groth16(),
        //     )
        //     .unwrap()
        //     .receipt;
        // receipt.verify(super::GEN_PLAYER_ID).unwrap();

        // println!("Proof verified");
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
