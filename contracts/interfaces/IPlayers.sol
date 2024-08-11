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

    /// @notice Represents a secp256k1 compressed public key
    struct Secp256k1PubKey {
        bytes[33] key;
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Events
    //  ─────────────────────────────────────────────────────────────────────────────

    event PackRequested(address indexed requester, uint256 indexed packId, Tier indexed tier);

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Minting Functions
    //  ─────────────────────────────────────────────────────────────────────────────

    /**
     * @notice Requests a pack be minted
     */
    function requestPack(Tier tier) external payable returns (uint256 packId);

    /**
     * @notice Allows fulfiller to fill a user's pack order
     */
    function fulfillPackOrder(uint256 orderId, string[15] calldata URIs, bytes calldata seal) external;

    /**
     * @notice Allows user to cancel a pack order if it has not been fulfilled in reasonable time
     */
    function cancelOrder(uint256 orderId) external;

    /**
     * @notice Returns the cost of a pack
     */
    function costOfPack(Tier tier) external pure returns (uint256 cost);

}
