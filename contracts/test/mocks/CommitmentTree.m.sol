// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {CommitmentTree} from "../../src/state/CommitmentTree.sol";

contract CommitmentTreeMock is CommitmentTree {
    function initialize() external initializer {
        __CommitmentTree_init();
    }

    function addCommitment(bytes32 commitment) external returns (bytes32 newRoot) {
        newRoot = _addCommitment(commitment);
    }

    function addCommitmentTreeRoot(bytes32 root) external {
        _addCommitmentTreeRoot(root);
    }
}
