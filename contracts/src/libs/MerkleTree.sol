// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Arrays} from "@openzeppelin-contracts-5.7.0/utils/Arrays.sol";

import {SHA256} from "../libs/SHA256.sol";

/// @title MerkleTree
/// @author Anoma Foundation, 2025
/// @notice A Merkle tree implementation populating a tree of variable depth from left to right.
/// @dev This is a modified version of the OpenZeppelin `MerkleTree` implementation
/// (https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v5.4.0/contracts/utils/structs/MerkleTree.sol).
/// @custom:security-contact security@anoma.foundation
library MerkleTree {
    struct Tree {
        uint256 _nextLeafIndex;
        bytes32[] _sides;
        bytes32[] _zeros;
    }

    /// @notice Sets up the tree with an initial capacity (i.e. number of leaves) of 1
    /// and returns the initial root of the empty tree.
    /// @param self The tree data structure.
    /// @return initialRoot The initial root of the empty tree.
    function setup(Tree storage self) internal returns (bytes32 initialRoot) {
        initialRoot = SHA256.EMPTY_HASH;

        // Store the root of the empty subtree of depth 0.
        self._zeros.push(SHA256.EMPTY_HASH);

        self._nextLeafIndex = 0;
    }

    /// @notice Pushes a leaf to the tree.
    /// @dev Only the level that the leaf completes is written; other levels may be stale and are read only where the
    /// count has a one bit. It computes no root; use `currentRoot` for that.
    /// @param self The tree data structure.
    /// @param leaf The leaf to add.
    function push(Tree storage self, bytes32 leaf) internal {
        // Get the next leaf index and increment it after assignment.
        uint256 index = self._nextLeafIndex++;

        // Fold the leaf with the left siblings (sides) while the node is a right child; the result is the root of the
        // subtree that the leaf completes. The level counts the trailing ones of the index.
        bytes32 node = leaf;
        uint256 level = 0;
        for (uint256 currentIndex = index; !isLeftChild(currentIndex); currentIndex >>= 1) {
            node = SHA256.hash(Arrays.unsafeAccess(self._sides, level).value, node);
            ++level;
        }

        if (level == depth(self)) {
            // The capacity is reached. The leaf finishes the whole tree, whose root becomes the side of a new level.
            self._sides.push(node);

            // Store the root of the empty subtree of the next depth.
            bytes32 currentZero = Arrays.unsafeAccess(self._zeros, level).value;
            self._zeros.push(SHA256.hash(currentZero, currentZero));
        } else {
            // NOTE: Only this level is written, the next `push` or `currentRoot` reads it.
            Arrays.unsafeAccess(self._sides, level).value = node;
        }
    }

    /// @notice Computes the root of the tree in its current state.
    /// @dev The roots of the empty subtrees are hashed while folding, which costs less than reading `_zeros`.
    /// @param self The tree data structure.
    /// @return treeRoot The root of the tree.
    function currentRoot(Tree storage self) internal view returns (bytes32 treeRoot) {
        uint256 treeDepth = depth(self);
        uint256 index = self._nextLeafIndex;

        // The root of the empty subtree of the current level.
        bytes32 zero = SHA256.EMPTY_HASH;
        treeRoot = zero;
        for (uint256 i = 0; i < treeDepth; ++i) {
            if (isLeftChild(index)) {
                treeRoot = SHA256.hash(treeRoot, zero);
            } else {
                treeRoot = SHA256.hash(Arrays.unsafeAccess(self._sides, i).value, treeRoot);
            }

            zero = SHA256.hash(zero, zero);
            index >>= 1;
        }
    }

    /// @notice Returns the tree depth.
    /// @param self The tree data structure.
    /// @return treeDepth The depth of the tree.
    function depth(Tree storage self) internal view returns (uint8 treeDepth) {
        // The tree gains one level per capacity doubling, so the depth cannot reach 256.
        // forge-lint: disable-next-line(unsafe-typecast)
        treeDepth = uint8(self._sides.length);
    }

    /// @notice Returns the number of leaves that have been added to the tree.
    /// @param self The tree data structure.
    /// @return count The number of leaves in the tree.
    function leafCount(Tree storage self) internal view returns (uint256 count) {
        count = self._nextLeafIndex;
    }

    /// @notice Returns the sides of the tree: for each level below the root, the root of the last complete left
    /// subtree on that level. `push` keeps them to add leaves without storing the earlier ones. Only the levels where
    /// the leaf count has a one bit are meaningful.
    /// @param self The tree data structure.
    /// @return treeSides The sides, from the leaf level up.
    function sides(Tree storage self) internal view returns (bytes32[] memory treeSides) {
        treeSides = self._sides;
    }

    /// @notice Returns the zeros of the tree: for each level up to the root, the root of an empty subtree of that
    /// height.
    /// @param self The tree data structure.
    /// @return treeZeros The zeros, from the leaf level up.
    function zeros(Tree storage self) internal view returns (bytes32[] memory treeZeros) {
        treeZeros = self._zeros;
    }

    /// @notice Calculates the capacity of the tree.
    /// @param self The tree data structure.
    /// @return treeCapacity The computed tree capacity.
    function capacity(Tree storage self) internal view returns (uint256 treeCapacity) {
        treeCapacity = uint256(1) << depth(self); // 2^treeDepth
    }

    /// @notice Checks whether a node is the left or right child according to its index.
    /// @param index The index to check.
    /// @return isLeft Whether this node is the left or right child.
    function isLeftChild(uint256 index) internal pure returns (bool isLeft) {
        isLeft = (index & 1) == 0;
    }
}
