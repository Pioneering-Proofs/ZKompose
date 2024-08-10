// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import "forge-std/console.sol";
import "forge-std/console2.sol";
import "forge-std/Test.sol";

import {Players, IPlayers} from "src/Players.sol";

contract TestPlayer is Test {

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Fields
    //  ─────────────────────────────────────────────────────────────────────────────

    Players public players;

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Setup
    //  ─────────────────────────────────────────────────────────────────────────────

    function setUp() public {
        players = new Players();
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Tests
    //  ─────────────────────────────────────────────────────────────────────────────

    function test_mintPlayer(bytes32 cid, uint256 tokenId) public {
        address owner;
        try players.ownerOf(tokenId) returns (address _owner) {
            owner = _owner;
        } catch {
            owner = address(0);
        }

        if (owner != address(0)) {
            return;
        }

        players.mintPlayer(tokenId, cid);

        assertEq(
            keccak256(abi.encodePacked(players.tokenURI(tokenId))),
            keccak256(abi.encodePacked("ipfs://", cid))
        );
        assertEq(players.ownerOf(tokenId), address(this));
    }

    function test_requestPack(uint8 tier) public {
        tier = uint8(bound(uint256(tier), 0, 4));
        IPlayers.Tier tierEnum = IPlayers.Tier(tier);
        IPlayers.Secp256k1PubKey memory key = IPlayers.Secp256k1PubKey(42, 42);
        uint256 cost = players.costOfPack(tierEnum);
        uint256 balanceBefore = address(this).balance;

        vm.expectEmit(address(players));
        emit IPlayers.PackRequested(address(this), players.currentPackId(), tierEnum, key);

        uint256 packId = players.requestPack{value: cost}(tierEnum, key);

        assertEq(address(this).balance, balanceBefore - cost);

        IPlayers.Secp256k1PubKey memory savedKey = players.pubKeys(address(this));
        console.log("savedKey.x", savedKey.x);
        console.log("savedKey.y", savedKey.y);
        assertEq(savedKey.x, key.x);

        Players.PackRequest memory packRequest = players.packRequests(packId);
        assertEq(uint256(packRequest.tier), uint256(tierEnum));
        assertEq(packRequest.requester, address(this));
    }

}
