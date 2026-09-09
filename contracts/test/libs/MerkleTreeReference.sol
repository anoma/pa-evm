// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Math} from "@openzeppelin-contracts-5.7.0/utils/math/Math.sol";
import {SafeCast} from "@openzeppelin-contracts-5.7.0/utils/math/SafeCast.sol";

import {SHA256} from "../../src/libs/SHA256.sol";

/// @notice Computes a Merkle root from a complete leaf set, the straightforward way. The protocol adapter never does
/// this: it maintains the root incrementally in `MerkleTree`. Tests use this to obtain a root the incremental
/// implementation must reproduce.
library MerkleTreeReference {
    using SafeCast for uint256;

    /// @notice Computes the root of a Merkle tree.
    /// @param leaves The leaves of the tree.
    /// @param treeDepth The depth of the tree.
    /// @return root The computed root.
    /// @dev This method should only be used for trees with low depth.
    function computeRoot(bytes32[] memory leaves, uint8 treeDepth) internal pure returns (bytes32 root) {
        uint256 treeCapacity = uint256(1) << treeDepth; // 2^treeDepth

        // Create array of full leaf set with padding if necessary
        bytes32[] memory nodes = new bytes32[](treeCapacity);
        for (uint256 i = 0; i < treeCapacity; ++i) {
            if (i < leaves.length) {
                nodes[i] = leaves[i];
            } else {
                nodes[i] = SHA256.EMPTY_HASH;
            }
        }

        // Build the tree upward
        uint256 currentLevelCapacity = treeCapacity;
        while (currentLevelCapacity > 1) {
            currentLevelCapacity /= 2;

            for (uint256 i = 0; i < currentLevelCapacity; ++i) {
                nodes[i] = SHA256.hash(nodes[2 * i], nodes[2 * i + 1]);
            }
        }

        root = nodes[0];
    }

    /// @notice Computes the root of a Merkle tree using the minimal tree depth to fit all leaves.
    /// @param leaves The leaves of the tree.
    /// @return root The computed root.
    /// @dev This method should only be used for trees with low depth.
    function computeRoot(bytes32[] memory leaves) internal pure returns (bytes32 root) {
        root = computeRoot({leaves: leaves, treeDepth: computeMinimalTreeDepth(leaves.length)});
    }

    /// @notice Computes the minimal required tree depth for a number of leaves.
    /// @param leavesCount The number of leaves.
    /// @return treeDepth The minimal required tree depth.
    function computeMinimalTreeDepth(uint256 leavesCount) internal pure returns (uint8 treeDepth) {
        treeDepth = Math.log2({value: leavesCount, rounding: Math.Rounding.Ceil}).toUint8();
    }
}
