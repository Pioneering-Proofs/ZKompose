// SPDX-License-Identifier: MIT

pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import "forge-std/Test.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {RiscZeroGroth16Verifier} from "risc0/groth16/RiscZeroGroth16Verifier.sol";
import {ControlID} from "risc0/groth16/ControlID.sol";

import {EvenNumber} from "../contracts/EvenNumber.sol";
import {Players} from "src/Players.sol";
import {IPlayers} from "src/interfaces/IPlayers.sol";
import {Team} from "src/Team.sol";

contract Mint is Script {

    string constant CONFIG_FILE = "script/config.toml";

    function run() external {
        Players players;

        uint256 chainId = block.chainid;
        console2.log("You are minting on ChainID %d", chainId);

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
            string memory configProfileKey = string.concat(".profile.", configProfile);
            address playersAddress =
                stdToml.readAddress(config, string.concat(configProfileKey, ".playersAddress"));
            players = Players(playersAddress);
        }

        if (address(players) == address(0)) {
            console2.log("Players contract not found in config");
            return;
        }

        uint256 packId = players.requestPack{value: 1 ether}(IPlayers.Tier.Diamond);
        console2.log("Requested pack with ID %d", packId);
    }

}
