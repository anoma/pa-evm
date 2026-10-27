// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Arrays} from "@openzeppelin-contracts-5.7.0/utils/Arrays.sol";

import {SHA256} from "../libs/SHA256.sol";

/// @title MerkleTree
/// @author Anoma Foundation, 2025
/// @notice A Merkle tree that adds leaves from left to right and increases its depth by one each time a leaf fills it.
/// @dev The tree stores its left siblings instead of its leaves.
/// @custom:security-contact security@anoma.foundation
library MerkleTree {
    struct Tree {
        uint256 _leafCount;
        bytes32[] _leftSiblings;
        bytes32[] _emptySubtreeRoots;
    }

    /// @notice Sets up an empty tree of depth 0 and returns its root.
    /// @param self The tree, which must not be set up yet.
    /// @return initialRoot The root of the empty tree.
    function setup(Tree storage self) internal returns (bytes32 initialRoot) {
        initialRoot = SHA256.EMPTY_HASH;

        // Store the empty subtree root of the leaf level.
        self._emptySubtreeRoots.push(SHA256.EMPTY_HASH);

        self._leafCount = 0;
    }

    /// @notice Adds a leaf to the tree.
    /// @param self The tree.
    /// @param leaf The leaf to add.
    /// @dev It writes one left sibling, the root of the subtree that the leaf completes, and computes no tree root.
    function push(Tree storage self, bytes32 leaf) internal {
        // The new leaf's index is the leaf count before the push.
        uint256 leafIndex = self._leafCount++;

        // The loop ends at the root of the subtree that the leaf completes.
        bytes32 node = leaf;
        uint256 level = 0;
        for (uint256 nodeIndex = leafIndex; isRightChild(nodeIndex); nodeIndex >>= 1) {
            node = SHA256.hash(Arrays.unsafeAccess(self._leftSiblings, level).value, node);
            ++level;
        }

        if (level == depth(self)) {
            // The leaf fills the tree, so the root of the tree becomes the left sibling on a new level.
            self._leftSiblings.push(node);

            // Add the empty subtree root of the level of the new root.
            bytes32 emptySubtreeRoot = Arrays.unsafeAccess(self._emptySubtreeRoots, level).value;
            self._emptySubtreeRoots.push(SHA256.hash(emptySubtreeRoot, emptySubtreeRoot));
        } else {
            // The node is a left child, so it is the left sibling of the next node on its level.
            Arrays.unsafeAccess(self._leftSiblings, level).value = node;
        }
    }

    /// @notice Computes the root of the tree.
    /// @param self The tree.
    /// @return treeRoot The root of the tree.
    /// @dev Hashing the empty subtree roots uses less gas than reading them from `_emptySubtreeRoots`.
    function currentRoot(Tree storage self) internal view returns (bytes32 treeRoot) {
        uint256 treeDepth = depth(self);
        uint256 nodeIndex = self._leafCount;

        // The empty subtree root of the current level.
        bytes32 emptySubtreeRoot = SHA256.EMPTY_HASH;
        bytes32 node = emptySubtreeRoot;
        for (uint256 level = 0; level < treeDepth; ++level) {
            if (isRightChild(nodeIndex)) {
                node = SHA256.hash(Arrays.unsafeAccess(self._leftSiblings, level).value, node);
            } else {
                node = SHA256.hash(node, emptySubtreeRoot);
            }

            emptySubtreeRoot = SHA256.hash(emptySubtreeRoot, emptySubtreeRoot);
            nodeIndex >>= 1;
        }
        treeRoot = node;
    }

    /// @notice Returns the depth of the tree.
    /// @param self The tree.
    /// @return treeDepth The number of levels below the root.
    function depth(Tree storage self) internal view returns (uint8 treeDepth) {
        // The tree gains one level per capacity doubling, so the depth cannot reach 256.
        // forge-lint: disable-next-line(unsafe-typecast)
        treeDepth = uint8(self._leftSiblings.length);
    }

    /// @notice Returns the number of leaves in the tree.
    /// @param self The tree.
    /// @return count The number of leaves, which is also the index of the next leaf.
    function leafCount(Tree storage self) internal view returns (uint256 count) {
        count = self._leafCount;
    }

    /// @notice Returns the left siblings of the tree, one per level below the root.
    /// @param self The tree.
    /// @return treeLeftSiblings The left siblings, from the leaf level up.
    /// @dev On each level where the leaf count has a one bit, the entry is the sibling on that level in the Merkle path
    /// of the next leaf. The other levels hold stale values.
    function leftSiblings(Tree storage self) internal view returns (bytes32[] memory treeLeftSiblings) {
        treeLeftSiblings = self._leftSiblings;
    }

    /// @notice Returns the empty subtree roots of the tree, one per level up to the root.
    /// @param self The tree.
    /// @return treeEmptySubtreeRoots The roots of subtrees without leaves, from the leaf level up.
    function emptySubtreeRoots(Tree storage self) internal view returns (bytes32[] memory treeEmptySubtreeRoots) {
        treeEmptySubtreeRoots = self._emptySubtreeRoots;
    }

    /// @notice Computes the capacity of the tree: the number of leaves that fill it at its depth.
    /// @param self The tree.
    /// @return treeCapacity Two to the power of the depth.
    function capacity(Tree storage self) internal view returns (uint256 treeCapacity) {
        treeCapacity = uint256(1) << depth(self);
    }

    /// @notice Checks whether a node is the right child of its parent, according to its index on its level.
    /// @param nodeIndex The index of the node on its level.
    /// @return isRight Whether the node is a right child.
    function isRightChild(uint256 nodeIndex) internal pure returns (bool isRight) {
        isRight = (nodeIndex & 1) == 1;
    }
}
