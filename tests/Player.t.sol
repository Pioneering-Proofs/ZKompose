// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import "forge-std/console.sol";
import "forge-std/console2.sol";
import "forge-std/Test.sol";

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";

import {Players, IPlayers} from "src/Players.sol";

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

    function test_requestPack(uint8 tier, bytes32 pubKey) public {
        tier = uint8(bound(uint256(tier), 0, 4));
        IPlayers.Tier tierEnum = IPlayers.Tier(tier);

        uint256 cost = players.costOfPack(tierEnum);
        uint256 balanceBefore = address(this).balance;

        bytes[33] memory keyBytes; // = new bytes[33];
        // for (uint256 i = 0; i < 32; i++) {
        //     keyBytes[i] = bytes1(pubKey[i]);
        // }
        // keyBytes[0] = bytes(0x1);
        IPlayers.Secp256k1PubKey memory key = IPlayers.Secp256k1PubKey(keyBytes);

        vm.expectEmit(address(players));
        emit IPlayers.PackRequested(address(this), players.currentPackId(), tierEnum, key);

        uint256 packId = players.requestPack{value: cost}(tierEnum, key);

        assertEq(address(this).balance, balanceBefore - cost);

        IPlayers.Secp256k1PubKey memory savedKey = players.pubKeys(address(this));
        // console.log("savedKey", savedKey);
        // assertEq(savedKey.x, key.x);

        Players.PackRequest memory packRequest = players.packRequests(packId);
        assertEq(uint256(packRequest.tier), uint256(tierEnum));
        assertEq(packRequest.requester, address(this));
    }

    function test_cancel() public {}
    function test_fulfill() public {}

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
