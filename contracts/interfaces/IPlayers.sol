// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import {IERC721} from "@openzeppelin/contracts/token/ERC721/IERC721.sol";

/**
 * @title Players Interface
 * @notice Standard interface for the player contract
 */
interface IPlayers is IERC721 {

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Custom Errors
    //  ─────────────────────────────────────────────────────────────────────────────

    /// @notice Reverted if a given token has already been minted
    error ERC721AlreadyMinted(uint256 tokenId);

    /// @notice Reverted if a player does not have enough funds to mint a pack
    error InsufficientFunds(uint256 cost, uint256 paid);

    /// @notice Reverted if a player requests a pack with an invalid tier
    error InvalidTier();

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Types
    //  ─────────────────────────────────────────────────────────────────────────────

    /// @notice Represents a player's rarity
    enum Tier {
        Diamond,
        Platinum,
        Gold,
        Silver,
        Bronze
    }

    /// @notice Simple eliptic curve point
    struct Secp256k1PubKey {
        uint256 x;
        uint256 y;
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Events
    //  ─────────────────────────────────────────────────────────────────────────────

    event PackRequested(
        address indexed requester, uint256 indexed packId, Tier indexed tier, Secp256k1PubKey key
    );

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Minting Functions
    //  ─────────────────────────────────────────────────────────────────────────────

    /**
     * @notice Requests a pack be minted
     */
    function requestPack(Tier tier, Secp256k1PubKey calldata key) external payable returns (uint256 packId);

    /**
     * @notice Returns the cost of a pack
     */
    function costOfPack(Tier tier) external pure returns (uint256 cost);

}
