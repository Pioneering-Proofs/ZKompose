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
    //  Types
    //  ─────────────────────────────────────────────────────────────────────────────

    struct PackRequest {
        Tier tier;
        uint40 timestamp;
        address requester;
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Fields
    //  ─────────────────────────────────────────────────────────────────────────────

    uint256 public currentPackId;

    mapping(address holder => Secp256k1PubKey pubKeys) private _pubKeys;

    mapping(uint256 packId => PackRequest packRequest) private _packRequests;

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Setup
    //  ─────────────────────────────────────────────────────────────────────────────

    constructor() ERC721("Players", "PLR") {}

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Minting Functions
    //  ─────────────────────────────────────────────────────────────────────────────

    /// @inheritdoc IPlayers
    function requestPack(Tier tier, Secp256k1PubKey calldata key) external payable returns (uint256 packId) {
        uint256 cost = costOfPack(tier);
        if (msg.value < cost) revert InsufficientFunds(cost, msg.value);

        packId = currentPackId;

        if (_pubKeys[msg.sender].x == 0 && _pubKeys[msg.sender].y == 0) {
            _pubKeys[msg.sender] = key;
        }

        _packRequests[packId] =
            PackRequest({tier: tier, timestamp: uint40(block.timestamp), requester: msg.sender});

        emit PackRequested(msg.sender, packId, tier, key);

        currentPackId++;
    }

    /// @inheritdoc IPlayers
    function costOfPack(Tier tier) public pure returns (uint256 cost) {
        if (tier == Tier.Diamond) return 1 ether;
        else if (tier == Tier.Platinum) return 0.5 ether;
        else if (tier == Tier.Gold) return 0.25 ether;
        else if (tier == Tier.Silver) return 0.1 ether;
        else if (tier == Tier.Bronze) return 0.05 ether;
        else revert InvalidTier();
    }

    function pubKeys(address holder) public view returns (Secp256k1PubKey memory) {
        return _pubKeys[holder];
    }

    function packRequests(uint256 packId) public view returns (PackRequest memory) {
        return _packRequests[packId];
    }

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
