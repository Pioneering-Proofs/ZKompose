// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol";

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
    //  Custom Errors
    //  ─────────────────────────────────────────────────────────────────────────────

    /// @notice Reverted if caller is not the pack fulfiller.
    error UnauthorizedFulfiller(address fulfiller);

    /// @notice Reverted if pack order is not found.
    error PackOrderNotFound(uint256 packId);

    /// @notice Reverted if order is too new to cancel
    error OrderTooNew(uint40 timestamp);

    /// @notice Reverted if caller is not the pack requester.
    error UnauthorizedCancel(address requester);

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Fields
    //  ─────────────────────────────────────────────────────────────────────────────

    /// @notice Number of packs issued. NOT token ID counter
    uint256 public currentPackId;

    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;

    /// @notice Address permitted to fulfill pack order requests.
    address public immutable packFulfiller;

    /// @notice Image ID of the fulfill pack order zkVM binary
    bytes32 public constant genPlayerImageId = ImageID.GEN_PLAYER_ID;

    /// @notice Pack requests by ID
    mapping(uint256 packId => PackRequest packRequest) private _packRequests;

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Setup
    //  ─────────────────────────────────────────────────────────────────────────────

    /// @param _fulfiller The address able to fulfill pack requests.
    constructor(IRiscZeroVerifier _verifier, address _fulfiller) ERC721("Players", "PLR") {
        verifier = _verifier;
        packFulfiller = _fulfiller;
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Minting Functions
    //  ─────────────────────────────────────────────────────────────────────────────

    /// @inheritdoc IPlayers
    function requestPack(Tier tier) public payable returns (uint256 packId) {
        uint256 cost = costOfPack(tier);
        if (msg.value < cost) revert InsufficientFunds(cost, msg.value);

        packId = currentPackId;
        _packRequests[packId] =
            PackRequest({tier: tier, timestamp: uint40(block.timestamp), requester: msg.sender});

        emit PackRequested(msg.sender, packId, tier);

        currentPackId++;
    }

    /// @inheritdoc IPlayers
    function fulfillPackOrder(uint256 orderId, string[15] calldata URIs, bytes calldata seal) external {
        if (msg.sender != packFulfiller) revert UnauthorizedFulfiller(msg.sender);

        PackRequest storage request = _packRequests[orderId];
        if (request.requester == address(0)) revert PackOrderNotFound(orderId);

        bytes memory journal = abi.encode(request.tier, orderId, URIs);
        verifier.verify(seal, genPlayerImageId, sha256(journal));

        for (uint256 i = 0; i < 15; i++) {
            _mint(request.requester, (orderId * 15) + i, URIs[i]);
            currentPackId++;
        }

        delete _packRequests[orderId];
    }

    /// @inheritdoc IPlayers
    function cancelOrder(uint256 orderId) external {
        PackRequest storage request = _packRequests[orderId];
        if (request.requester != msg.sender) revert UnauthorizedCancel(msg.sender);
        if (request.timestamp + 1 days > block.timestamp) revert OrderTooNew(request.timestamp);

        // NOTE: Delete before transfer to prevent re-entrancy. Important that this is done first.
        delete _packRequests[orderId];

        payable(msg.sender).transfer(costOfPack(request.tier));
    }

    /// @inheritdoc IPlayers
    function costOfPack(Tier tier) public pure returns (uint256 cost) {
        if (tier == Tier.Diamond) return 0.0005 ether;
        else if (tier == Tier.Platinum) return 0.004 ether;
        else if (tier == Tier.Gold) return 0.0003 ether;
        else if (tier == Tier.Silver) return 0.0002 ether;
        else if (tier == Tier.Bronze) return 0.0001 ether;
        else revert InvalidTier();
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
