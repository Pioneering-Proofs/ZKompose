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

        players.requestPack{value: cost}(tierEnum);
        uint256 packId = players.requestPack{value: cost}(tierEnum);

        uint8 stdDev = 10;

        uint64 u = 12_312;
        uint64 v = 4242;

        console2.log("Fulfilling pack %d", packId);

        (bytes memory journal, bytes memory seal) =
            prove(Elf.GEN_PLAYER_PATH, abi.encode(packId, stdDev, tier, u, v));

        console2.log("Decoding journal of length: %d", journal.length);

        // Actual length 824

        // length: 32
        // decodedTier: 32,
        // decodedOrderId: 32,
        // cids: 46 * 15 = 690
        // total 786. 38 bytes, (aka 32 and 8) bytes off

        (uint8 decodedTier, uint256 decodedOrderId, bytes[46][15] memory cids) =
            abi.decode(journal, (uint8, uint256, bytes[46][15]));

        console2.log("Decoded journal: %d, %d", decodedTier, decodedOrderId);

        // bytes memory journalBytes = abi.decode(journal, (bytes));

        // (uint8 decodedTier,,) = abi.decode(journalBytes, (uint8, uint256, string[15]));

        // console2.log("Decoded tier: %d", decodedTier);
        // assertEq(decodedTier, tier);

        // (, uint256 decodedPackId, string[] memory URIs) = abi.decode(journal, (uint8, uint256, string[]));

        // assertEq(decodedTier, tier);
        // assertEq(decodedPackId, packId);
        // assertEq(URIs.length, 15);

        // for (uint256 i = 0; i < 15; i++) {
        //     console2.log("Minting token %d with URI %s", (packId * 15) + i, URIs[i]);
        // }

        // vm.prank(fulfiller);
        // players.fulfillPackOrder(packId, URIs, seal);

        // order_id: 42,
        //     std_dev: 10,
        //     tier: 1,
        //     u: 3.14159,
        //     v: 2123.71828,
    }

    // function test_mintPlayer(bytes32 cid, uint256 tokenId) public {
    //     address owner;
    //     try players.ownerOf(tokenId) returns (address _owner) {
    //         owner = _owner;
    //     } catch {
    //         owner = address(0);
    //     }

    //     if (owner != address(0)) {
    //         return;
    //     }

    //     players.mintPlayer(tokenId, cid);

    //     assertEq(
    //         keccak256(abi.encodePacked(players.tokenURI(tokenId))),
    //         keccak256(abi.encodePacked("ipfs://", cid))
    //     );
    //     assertEq(players.ownerOf(tokenId), address(this));
    // }

}
