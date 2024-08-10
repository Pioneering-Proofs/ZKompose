// SPDX-License-Identifier: MIT

pragma solidity ^0.8.24;

import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {ERC721Enumerable} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {IERC4906} from "@openzeppelin/contracts/interfaces/IERC4906.sol";
import {IERC165} from "@openzeppelin/contracts/interfaces/IERC165.sol";

abstract contract ERC721EnumerableURI is ERC721Enumerable, IERC4906 {

    using Strings for uint256;

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Fields
    //  ─────────────────────────────────────────────────────────────────────────────

    // Interface ID as defined in ERC-4906. This does not correspond to a traditional
    // only defines events and does not include any external function.
    bytes4 private constant ERC4906_INTERFACE_ID = bytes4(0x49064906);

    // Optional mapping for token URIs
    mapping(uint256 tokenId => string) private _tokenURIs;

    //  ──────────────────────────  Player View Functions  ──────────────────────────  \\

    /**
     * @dev See {IERC721Metadata-tokenURI}.
     */
    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        _requireOwned(tokenId);

        string memory _tokenURI = _tokenURIs[tokenId];
        string memory base = _baseURI();

        // If there is no base URI, return the token URI.
        if (bytes(base).length == 0) {
            return _tokenURI;
        }
        // If both are set, concatenate the baseURI and tokenURI (via string.concat).
        if (bytes(_tokenURI).length > 0) {
            return string.concat(base, _tokenURI);
        }

        return super.tokenURI(tokenId);
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
    function _setTokenURI(uint256 tokenId, string memory _tokenURI) internal virtual {
        _tokenURIs[tokenId] = _tokenURI;
        emit MetadataUpdate(tokenId);
    }

    /**
     * @dev Mints and sets a token URI
     */
    function _mint(address to, uint256 tokenId, string memory _tokenURI) internal virtual {
        _mint(to, tokenId);
        _setTokenURI(tokenId, _tokenURI);
    }

    //  ─────────────────────────────────────────────────────────────────────────────
    //  Modifiers
    //  ─────────────────────────────────────────────────────────────────────────────

    modifier onlyAuthorized(uint256 tokenId) {
        _checkAuthorized(_ownerOf(tokenId), msg.sender, tokenId);
        _;
    }

}
