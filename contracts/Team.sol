// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol";

import {IPlayer} from "./interfaces/IPlayer.sol";

import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {ERC721EnumerableURI} from "./extensions/ERC721EnumerableURI.sol";
import {ERC721Holder} from "@openzeppelin/contracts/token/ERC721/utils/ERC721Holder.sol";

/**
 * @title Team
 * @notice Manages a team of soccer players.
 * @author Kai Aldag
 */
contract Team is ERC721Holder, ERC721EnumerableURI {

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Custom Errors
    //  ─────────────────────────────────────────────────────────────────────────────

    /// @notice Reverted if this contract is not approved to use given player
    error PlayerApprovalRequired(address owner);

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Fields
    //  ─────────────────────────────────────────────────────────────────────────────

    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;

    /// @notice Address of the Player NFT contract
    IPlayer public immutable player;

    /// @notice Image ID of the team building zkVM binary
    bytes32 public constant buildTeamImageId = ImageID.BUILD_TEAM;

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Setup
    //  ─────────────────────────────────────────────────────────────────────────────

    constructor(IRiscZeroVerifier _verifier, IPlayer _player) ERC721("Teams", "TM") {
        verifier = _verifier;
        player = _player;
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Write Functions
    //  ─────────────────────────────────────────────────────────────────────────────

    /**
     */
    function buildTeam(uint256[5] calldata playerIds, bytes32 teamURI, bytes calldata seal) public {
        _checkApproval(msg.sender);

        for (uint256 i = 0; i < 5; i++) {
            _isAuthorized(playerIds[i], msg.sender);
        }

        bytes memory journal = abi.encode(playerIds, teamURI);
        verifier.verify(seal, buildTeamImageId, sha256(journal));
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Internal Utilities
    //  ─────────────────────────────────────────────────────────────────────────────

    /// @dev Checks the user is authorized to transfer a given tokenId. Must be owner or approved for all
    function _isAuthorized(uint256 tokenId, address user) private returns (bool isAuthorized) {
        address owner = player.ownerOf(tokenId);
        return player.ownerOf(tokenId) == user || player.isApprovedForAll(owner, user);
    }

    /// @dev Checks this contract is approved to use the caller's Player NFTs
    function _checkApproval(address caller) private {
        require(player.isApprovedForAll(caller, address(this)), PlayerApprovalRequired(caller));
    }

}
