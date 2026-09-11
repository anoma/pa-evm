// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.16.2/src/Test.sol";

import {MerkleTree} from "../../src/libs/MerkleTree.sol";
import {SeededTree} from "../../src/libs/SeededTree.sol";

/// @dev Both functions read a tree that `MerkleTree` grew, so every check compares them against what `push` and
/// `setup` produced.
contract SeededTreeTest is Test {
    using MerkleTree for MerkleTree.Tree;
    using SeededTree for MerkleTree.Tree;

    uint256 internal constant _LEAF_COUNT = 20;

    MerkleTree.Tree internal _tree;

    function test_currentRoot_matches_the_root_returned_by_every_push() public {
        assertEq(_tree.setup(), _tree.currentRoot(), "the empty tree's root differs");

        for (uint256 i = 0; i < _LEAF_COUNT; ++i) {
            (, bytes32 pushed) = _tree.push(_leaf(i));
            assertEq(_tree.currentRoot(), pushed, "the recomputed root differs after a push");
        }
    }

    function test_zeroHashes_match_the_zeros_the_tree_stores() public {
        _tree.setup();
        for (uint256 i = 0; i < _LEAF_COUNT; ++i) {
            _tree.push(_leaf(i));
        }

        bytes32[] memory hashes = SeededTree.zeroHashes(_tree.depth());

        assertEq(hashes.length, _tree._zeros.length, "zero hash count differs");
        for (uint256 i = 0; i < hashes.length; ++i) {
            assertEq(hashes[i], _tree._zeros[i], "a zero hash differs");
        }
    }

    function _leaf(uint256 index) internal pure returns (bytes32 leaf) {
        leaf = keccak256(abi.encode(index));
    }
}
