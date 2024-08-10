// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import {console} from "forge-std/console.sol";
import {console2} from "forge-std/console2.sol";
import {Test} from "forge-std/Test.sol";

import {Players} from "src/Players.sol";

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

}
