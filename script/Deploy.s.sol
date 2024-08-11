// Copyright 2024 RISC Zero, Inc.
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
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import "forge-std/Test.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {RiscZeroGroth16Verifier} from "risc0/groth16/RiscZeroGroth16Verifier.sol";
import {ControlID} from "risc0/groth16/ControlID.sol";

import {EvenNumber} from "../contracts/EvenNumber.sol";
import {Players} from "src/Players.sol";
import {Team} from "src/Team.sol";

/// @notice Deployment script for the RISC Zero starter project.
/// @dev Use the following environment variable to control the deployment:
///     * Set one of these two environment variables to control the deployment wallet:
///         * ETH_WALLET_PRIVATE_KEY private key of the wallet account.
///         * ETH_WALLET_ADDRESS address of the wallet account.
///
/// See the Foundry documentation for more information about Solidity scripts,
/// including information about wallet options.
///
/// https://book.getfoundry.sh/tutorials/solidity-scripting
/// https://book.getfoundry.sh/reference/forge/forge-script
contract EvenNumberDeploy is Script {

    // Path to deployment config file, relative to the project root.
    string constant CONFIG_FILE = "script/config.toml";

    IRiscZeroVerifier verifier;

    function run() external {
        // Read and log the chainID
        uint256 chainId = block.chainid;
        console2.log("You are deploying on ChainID %d", chainId);

        address fulfillerAddress;

        // Read the config profile from the environment variable, or use the default for the chainId.
        // Default is the first profile with a matching chainId field.
        string memory config = vm.readFile(string.concat(vm.projectRoot(), "/", CONFIG_FILE));
        string memory configProfile = vm.envOr("CONFIG_PROFILE", string(""));
        if (bytes(configProfile).length == 0) {
            string[] memory profileKeys = vm.parseTomlKeys(config, ".profile");
            for (uint256 i = 0; i < profileKeys.length; i++) {
                if (
                    stdToml.readUint(config, string.concat(".profile.", profileKeys[i], ".chainId"))
                        == chainId
                ) {
                    configProfile = profileKeys[i];
                    break;
                }
            }
        }

        if (bytes(configProfile).length != 0) {
            console2.log("Deploying using config profile:", configProfile);
            string memory configProfileKey = string.concat(".profile.", configProfile);
            address riscZeroVerifierAddress =
                stdToml.readAddress(config, string.concat(configProfileKey, ".riscZeroVerifierAddress"));
            // If set, use the predeployed verifier address found in the config.
            verifier = IRiscZeroVerifier(riscZeroVerifierAddress);

            fulfillerAddress =
                stdToml.readAddress(config, string.concat(configProfileKey, ".fulfillerAddress"));
        }

        vm.startBroadcast();

        // Deploy the verifier, if not already deployed.
        if (address(verifier) == address(0)) {
            verifier = new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
            console2.log("Deployed RiscZeroGroth16Verifier to", address(verifier));
        } else {
            console2.log("Using IRiscZeroVerifier contract deployed at", address(verifier));
        }

        if (address(fulfillerAddress) == address(0)) vm.envAddress("FULFILLER_ADDRESS");
        if (address(fulfillerAddress) == address(0)) vm.envAddress("ETH_FROM");
        if (address(fulfillerAddress) == address(0)) {
            console2.log(
                "No fulfiller address provided. Please set FULFILLER_ADDRESS or ETH_FROM env vars or add to config.toml."
            );
            vm.stopBroadcast();
            return;
        }

        // Deploy the application contract.
        Players players = new Players(verifier, fulfillerAddress);
        console2.log("Deployed Players to", address(players));

        Team team = new Team(verifier, players);
        console2.log("Deployed Team to", address(team));

        vm.stopBroadcast();
    }

}
