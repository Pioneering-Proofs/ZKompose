// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {ERC721Enumerable} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {IERC4906} from "@openzeppelin/contracts/interfaces/IERC4906.sol";
import {IERC165} from "@openzeppelin/contracts/interfaces/IERC165.sol";
import {Base58} from "Base58/Base58.sol";

abstract contract ERC721EnumerableURI is ERC721Enumerable, IERC4906 {

    using Strings for uint256;

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Fields
    //  ─────────────────────────────────────────────────────────────────────────────

    // Interface ID as defined in ERC-4906. This does not correspond to a traditional
    // only defines events and does not include any external function.
    bytes4 private constant ERC4906_INTERFACE_ID = bytes4(0x49064906);

    // Optional mapping for token URIs
    mapping(uint256 tokenId => bytes32) private _tokenCIDs;

    //  ──────────────────────────  Player View Functions  ──────────────────────────  \\

    /**
     * @dev See {IERC721Metadata-tokenURI}.
     */
    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        _requireOwned(tokenId);

        bytes32 _tokenCID = _tokenCIDs[tokenId];
        string memory base = _baseURI();
        return string.concat(base, cidv0(_tokenCID));
    }

    //  ───────────────────────────  ERC-165 Conformance  ───────────────────────────  \\

    /**
     * @dev See {IERC165-supportsInterface}
     */
    function supportsInterface(bytes4 interfaceId)
        public
        view
        virtual
        override(ERC721Enumerable, IERC165)
        returns (bool)
    {
        return interfaceId == ERC4906_INTERFACE_ID || super.supportsInterface(interfaceId);
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Internal Utilities
    //  ─────────────────────────────────────────────────────────────────────────────

    /**
     * @dev Sets `_tokenURI` as the tokenURI of `tokenId`.
     *
     * Emits {MetadataUpdate}.
     */
    function _setTokenURI(uint256 tokenId, bytes32 _tokenCID) internal virtual {
        _tokenCIDs[tokenId] = _tokenCID;
        emit MetadataUpdate(tokenId);
    }

    /**
     * @dev Mints and sets a token URI
     */
    function _mint(address to, uint256 tokenId, bytes32 _tokenCID) internal virtual {
        _mint(to, tokenId);
        _setTokenURI(tokenId, _tokenCID);
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Encoding
    //  ─────────────────────────────────────────────────────────────────────────────

    function cidv0(bytes32 sha256Hash) internal view returns (string memory) {
        // TODO: Use bytelib to concatenate bytes more efficiently
        bytes memory hashString = new bytes(34);
        hashString[0] = 0x12;
        hashString[1] = 0x20;
        uint256 hashLength = sha256Hash.length;
        for (uint256 i = 0; i < hashLength; ++i) {
            hashString[i + 2] = sha256Hash[i];
        }
        return Base58.encodeToString(hashString);
    }

    function _baseURI() internal pure override returns (string memory) {
        return "ipfs://";
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Modifiers
    //  ─────────────────────────────────────────────────────────────────────────────

    modifier onlyAuthorized(uint256 tokenId) {
        _checkAuthorized(_ownerOf(tokenId), msg.sender, tokenId);
        _;
    }

}
