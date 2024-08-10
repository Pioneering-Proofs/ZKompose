// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";

/**
 * @title Player Interface
 * @notice Standard interface for the player contract
 */
interface IPlayer is IERC721 {

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Custom Errors
    //  ─────────────────────────────────────────────────────────────────────────────

    /// @notice Reverted if a given token has already been minted
    error ERC721AlreadyMinted(uint256 tokenId);

}
