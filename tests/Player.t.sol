// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import "forge-std/console.sol";
import "forge-std/console2.sol";
import "forge-std/Test.sol";

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";

import {Players, IPlayers} from "src/Players.sol";
import {Elf} from "./Elf.sol";

contract TestPlayer is RiscZeroCheats, Test {

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Fields
    //  ─────────────────────────────────────────────────────────────────────────────

    Players public players;

    address public fulfiller;

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Setup
    //  ─────────────────────────────────────────────────────────────────────────────

    function setUp() public {
        IRiscZeroVerifier verifier = deployRiscZeroVerifier();
        fulfiller = makeAddr("FULFILLER");
        vm.label(fulfiller, "fulfiller");
        players = new Players(verifier, fulfiller);
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Tests
    //  ─────────────────────────────────────────────────────────────────────────────

    function test_requestPack(uint8 tier) public {
        tier = uint8(bound(uint256(tier), 0, 4));
        IPlayers.Tier tierEnum = IPlayers.Tier(tier);

        uint256 cost = players.costOfPack(tierEnum);
        uint256 balanceBefore = address(this).balance;

        vm.expectEmit(address(players));
        emit IPlayers.PackRequested(address(this), players.currentPackId(), tierEnum);

        uint256 packId = players.requestPack{value: cost}(tierEnum);

        assertEq(address(this).balance, balanceBefore - cost);

        Players.PackRequest memory packRequest = players.packRequests(packId);
        assertEq(uint256(packRequest.tier), uint256(tierEnum));
        assertEq(packRequest.requester, address(this));
    }

    function test_cancel() public {}

    function test_fulfill() public {
        // tier = uint8(bound(uint256(tier), 0, 4));
        uint8 tier = 1; // NOTE: Fuzzing isn't really an option here
        IPlayers.Tier tierEnum = IPlayers.Tier(tier);
        uint256 cost = players.costOfPack(tierEnum);

        uint256 packId = players.requestPack{value: cost}(tierEnum);

        uint64 u = 12_312;
        uint64 v = 4242;

        console2.log("Fulfilling pack %d", packId);

        (bytes memory journal, bytes memory seal) = prove(Elf.GEN_PLAYER_PATH, abi.encode(tier, packId, u, v));

        console2.log("Decoding journal of length: %d", journal.length);

        (uint8 decodedTier, uint256 decodedOrderId, bytes32[15] memory cids) =
            abi.decode(journal, (uint8, uint256, bytes32[15]));

        console2.log("Decoded journal: %d, %d", decodedTier, decodedOrderId);

        assertEq(tier, decodedTier);
        assertEq(packId, decodedOrderId);

        vm.prank(fulfiller);
        players.fulfillPackOrder(packId, cids, seal);

        for (uint256 i = 0; i < 15; i++) {
            console2.log("CID %d: %s", i, players.tokenURI(i));
        }
    }

}
