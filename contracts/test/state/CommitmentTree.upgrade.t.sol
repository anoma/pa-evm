// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";
import {UnsafeUpgrades} from "openzeppelin-foundry-upgrades-0.4.2/src/Upgrades.sol";

import {SHA256} from "../../src/libs/SHA256.sol";
import {MerkleTreeReference} from "../libs/MerkleTreeReference.sol";
import {CommitmentTreeLegacyMock, CommitmentTreeUpgradeMock} from "../mocks/CommitmentTreeUpgrade.m.sol";

/// @notice Upgrades a proxy holding a tree written by the legacy `push` to the current implementation, and keeps
/// pushing. The roots must stay those of the padded tree, so the live trees survive the upgrade.
contract CommitmentTreeUpgradeTest is Test {
    /// @dev A table test, so that each count runs in a fresh execution.
    function tableSmallCounts_roots_stay_those_of_the_padded_tree_across_the_upgrade(uint256 smallCount) public {
        _pushUpgradePush({beforeCount: smallCount, afterCount: 70, checkSides: true});
    }

    function tableBoundaryCounts_roots_stay_those_of_the_padded_tree_across_the_upgrade(uint256 boundaryCount) public {
        _pushUpgradePush({beforeCount: boundaryCount, afterCount: 5, checkSides: false});
    }

    function fixtureSmallCount() public pure returns (uint256[] memory counts) {
        counts = new uint256[](71);
        for (uint256 i = 0; i < counts.length; ++i) {
            counts[i] = i;
        }
    }

    /// @dev Around each power of two from 128 to 1,024, and the count of the Base production tree.
    function fixtureBoundaryCount() public pure returns (uint256[] memory counts) {
        counts = new uint256[](21);
        uint256 n = 0;
        for (uint256 p = 128; p <= 1024; p <<= 1) {
            for (uint256 count = p - 2; count <= p + 2; ++count) {
                counts[n++] = count;
            }
        }
        counts[n] = 2824;
    }

    /// @dev Pushes `beforeCount` leaves through the legacy implementation, upgrades, pushes `afterCount` more, and
    /// compares the root with the padded-tree root after every push.
    function _pushUpgradePush(uint256 beforeCount, uint256 afterCount, bool checkSides) internal {
        address proxy = UnsafeUpgrades.deployUUPSProxy(
            address(new CommitmentTreeLegacyMock()), abi.encodeCall(CommitmentTreeLegacyMock.initialize, ())
        );

        bytes32[] memory leaves = new bytes32[](beforeCount + afterCount);
        for (uint256 i = 0; i < leaves.length; ++i) {
            leaves[i] = keccak256(abi.encode("leaf", i));
        }

        for (uint256 i = 0; i < beforeCount; ++i) {
            CommitmentTreeLegacyMock(proxy).addCommitment(leaves[i]);
        }
        assertEq(CommitmentTreeLegacyMock(proxy).latestCommitmentTreeRoot(), _root(leaves, beforeCount), "legacy root");

        UnsafeUpgrades.upgradeProxy(proxy, address(new CommitmentTreeUpgradeMock()), "");

        CommitmentTreeUpgradeMock tree = CommitmentTreeUpgradeMock(proxy);
        assertEq(tree.latestCommitmentTreeRoot(), _root(leaves, beforeCount), "root after the upgrade");
        assertEq(tree.commitmentCount(), beforeCount, "count after the upgrade");

        for (uint256 i = beforeCount; i < leaves.length; ++i) {
            tree.addCommitment(leaves[i]);
            assertEq(tree.latestCommitmentTreeRoot(), _root(leaves, i + 1), "root after a push");
            _assertViews(tree, leaves, i + 1, checkSides);
        }
    }

    /// @dev The stored arrays keep their meaning: one side per level below the root and one zero per level up to it,
    /// and the side of a level where the leaf count has a one bit is the root of the complete left subtree there.
    function _assertViews(CommitmentTreeUpgradeMock tree, bytes32[] memory leaves, uint256 count, bool checkSides)
        internal
        view
    {
        bytes32[] memory sides = tree.commitmentTreeSides();
        bytes32[] memory zeros = tree.commitmentTreeZeros();

        uint256 treeDepth = 0;
        for (uint256 x = count; x > 0; x >>= 1) {
            ++treeDepth;
        }
        assertEq(tree.commitmentTreeDepth(), treeDepth, "depth");
        assertEq(sides.length, treeDepth, "the number of sides");
        assertEq(zeros.length, treeDepth + 1, "the number of zeros");
        assertEq(zeros[0], SHA256.EMPTY_HASH, "the first zero");
        for (uint256 level = 1; level < zeros.length; ++level) {
            assertEq(zeros[level], SHA256.hash(zeros[level - 1], zeros[level - 1]), "a zero");
        }

        if (!checkSides) {
            return;
        }
        for (uint256 level = 0; level < treeDepth; ++level) {
            if (((count >> level) & 1) == 1) {
                uint256 start = (count >> (level + 1)) << (level + 1);
                bytes32[] memory subtree = new bytes32[](uint256(1) << level);
                for (uint256 j = 0; j < subtree.length; ++j) {
                    subtree[j] = leaves[start + j];
                }
                assertEq(sides[level], MerkleTreeReference.computeRoot(subtree, uint8(level)), "a complete subtree");
            }
        }
    }

    function _root(bytes32[] memory leaves, uint256 count) internal pure returns (bytes32 root) {
        bytes32[] memory prefix = new bytes32[](count);
        for (uint256 i = 0; i < count; ++i) {
            prefix[i] = leaves[i];
        }
        root = MerkleTreeReference.computeCommitmentTreeRoot(prefix);
    }
}
