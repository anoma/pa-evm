// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SHA256} from "./SHA256.sol";

/// @title SeededTree
/// @author Anoma Foundation, 2026
/// @notice Computes the empty-subtree roots of a commitment tree that is written directly instead of grown by
/// `MerkleTree.push`. The protocol adapter never writes a tree that way, so `MerkleTree` does not carry the function,
/// and only `TransitionalProtocolAdapter` uses it.
/// @custom:security-contact security@anoma.foundation
library SeededTree {
    /// @notice Returns the roots of the empty subtrees, one per level, for a tree of the given depth.
    /// @param treeDepth The depth of the tree.
    /// @return hashes The empty-subtree roots, from level 0 up to `treeDepth`.
    /// @dev `MerkleTree.setup` stores the first and `MerkleTree.push` appends one per level it adds, so this
    /// reproduces what a tree of that depth holds.
    function zeroHashes(uint8 treeDepth) internal pure returns (bytes32[] memory hashes) {
        hashes = new bytes32[](uint256(treeDepth) + 1);
        hashes[0] = SHA256.EMPTY_HASH;

        for (uint256 i = 0; i < treeDepth; ++i) {
            hashes[i + 1] = SHA256.hash(hashes[i], hashes[i]);
        }
    }
}
