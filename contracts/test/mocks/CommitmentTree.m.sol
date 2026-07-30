// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {EnumerableSet} from "@openzeppelin-contracts-5.6.1/utils/structs/EnumerableSet.sol";

import {MerkleTree} from "../../src/libs/MerkleTree.sol";
import {CommitmentTree} from "../../src/state/CommitmentTree.sol";

contract CommitmentTreeMock is CommitmentTree {
    using EnumerableSet for EnumerableSet.Bytes32Set;
    using MerkleTree for MerkleTree.Tree;

    function initialize() external initializer {
        __CommitmentTree_init();
    }

    function addCommitment(bytes32 commitment) external returns (bytes32 newRoot) {
        newRoot = _addCommitment(commitment);
    }

    function addCommitmentTreeRoot(bytes32 root) external {
        _addCommitmentTreeRoot(root);
    }

    function initialRoot() external view returns (bytes32 hash) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        hash = $._roots.at(0);
    }
}
