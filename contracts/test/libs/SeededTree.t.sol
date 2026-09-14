// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.16.2/src/Test.sol";

import {MerkleTree} from "../../src/libs/MerkleTree.sol";
import {SeededTree} from "../../src/libs/SeededTree.sol";

/// @dev The zero hashes are compared against the zeros that `setup` and `push` stored in a grown tree.
contract SeededTreeTest is Test {
    using MerkleTree for MerkleTree.Tree;

    uint256 internal constant _LEAF_COUNT = 20;

    MerkleTree.Tree internal _tree;

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
