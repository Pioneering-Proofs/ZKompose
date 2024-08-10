// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import {IPlayers} from "./interfaces/IPlayers.sol";

import {ERC721EnumerableURI} from "./extensions/ERC721EnumerableURI.sol";
import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";

/**
 * @title Players
 * @notice Records soccer player NFTs.
 * @author Kai Aldag
 */
contract Players is ERC721EnumerableURI, IPlayers {

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Setup
    //  ─────────────────────────────────────────────────────────────────────────────

    constructor() ERC721("Players", "PLR") {}

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Internal Utils
    //  ─────────────────────────────────────────────────────────────────────────────

    // TODO: Implement this in robust way. Using this for rapid testing
    function mintPlayer(uint256 tokenId, bytes32 cid) public payable {
        if (_ownerOf(tokenId) != address(0)) revert ERC721AlreadyMinted(tokenId);

        _mint(msg.sender, tokenId, string(abi.encodePacked(cid)));
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Internal Utils
    //  ─────────────────────────────────────────────────────────────────────────────

    function _baseURI() internal pure override returns (string memory) {
        return "ipfs://";
    }

}
