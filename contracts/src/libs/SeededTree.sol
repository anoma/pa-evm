// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Arrays} from "@openzeppelin-contracts-5.7.0/utils/Arrays.sol";

import {MerkleTree} from "./MerkleTree.sol";
import {SHA256} from "./SHA256.sol";

/// @title SeededTree
/// @author Anoma Foundation, 2026
/// @notice Reads a commitment tree that was written directly instead of grown by `MerkleTree.push`. The protocol
/// adapter never does that, so `MerkleTree` carries neither function; both live here, with the transition that needs
/// them.
/// @custom:security-contact security@anoma.foundation
library SeededTree {
    /// @notice Computes the root of the tree in its current state, without adding a leaf.
    /// @param self The tree data structure.
    /// @return treeRoot The root of the tree.
    /// @dev Every leaf at or after the next leaf index is empty, so the stored sides and the zeros give the root.
    function currentRoot(MerkleTree.Tree storage self) internal view returns (bytes32 treeRoot) {
        uint256 treeDepth = MerkleTree.depth(self);
        uint256 index = self._nextLeafIndex;

        treeRoot = Arrays.unsafeAccess(self._zeros, 0).value;
        for (uint256 i = 0; i < treeDepth; ++i) {
            if (MerkleTree.isLeftChild(index)) {
                treeRoot = SHA256.hash(treeRoot, Arrays.unsafeAccess(self._zeros, i).value);
            } else {
                treeRoot = SHA256.hash(Arrays.unsafeAccess(self._sides, i).value, treeRoot);
            }

            index >>= 1;
        }
    }

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
