// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Ownable} from "@openzeppelin-contracts-5.7.0/access/Ownable.sol";
import {Pausable} from "@openzeppelin-contracts-5.7.0/utils/Pausable.sol";
import {EnumerableSet} from "@openzeppelin-contracts-5.7.0/utils/structs/EnumerableSet.sol";

import {MerkleTree} from "../../src/libs/MerkleTree.sol";

/// @notice A stand-in for the v1 protocol adapter, holding its state in v1's storage layout and exposing the getters
/// `MigrateProtocolAdapterState` reads. v1 inherits `ReentrancyGuardTransient, Ownable, Pausable, CommitmentTree,
/// NullifierSet`; the guard keeps its state in transient storage, so `Ownable._owner` is slot 0, `Pausable._paused`
/// is slot 1, and the tree follows at slots 2 to 4. The state variables below reproduce that order.
contract ProtocolAdapterV1Mock is Ownable, Pausable {
    using EnumerableSet for EnumerableSet.Bytes32Set;
    using MerkleTree for MerkleTree.Tree;

    MerkleTree.Tree internal _merkleTree; // slots 2, 3, 4
    EnumerableSet.Bytes32Set internal _roots; // slots 5, 6
    EnumerableSet.Bytes32Set internal _nullifierSet; // slots 7, 8

    constructor(address initialOwner) Ownable(initialOwner) {
        // slither-disable-next-line unused-return
        _roots.add(_merkleTree.setup());
    }

    function addCommitment(bytes32 commitment) external {
        (, bytes32 newRoot) = _merkleTree.push(commitment);
        // slither-disable-next-line unused-return
        _roots.add(newRoot);
    }

    function addNullifier(bytes32 nullifier) external {
        // slither-disable-next-line unused-return
        _nullifierSet.add(nullifier);
    }

    function emergencyStop() external onlyOwner whenNotPaused {
        _pause();
    }

    function commitmentCount() external view returns (uint256 count) {
        count = _merkleTree.leafCount();
    }

    function commitmentTreeDepth() external view returns (uint8 treeDepth) {
        treeDepth = _merkleTree.depth();
    }

    function commitmentTreeCapacity() external view returns (uint256 treeCapacity) {
        treeCapacity = _merkleTree.capacity();
    }

    /// @notice The stored sides of the tree, which the real v1 protocol adapter does not expose.
    function commitmentTreeSides() external view returns (bytes32[] memory sides) {
        sides = _merkleTree._sides;
    }

    function latestCommitmentTreeRoot() external view returns (bytes32 root) {
        root = _roots.at(_roots.length() - 1);
    }

    function nullifierCount() external view returns (uint256 count) {
        count = _nullifierSet.length();
    }

    function nullifierAtIndex(uint256 index) external view returns (bytes32 nullifier) {
        nullifier = _nullifierSet.at(index);
    }

    function isNullifierContained(bytes32 nullifier) external view returns (bool isContained) {
        isContained = _nullifierSet.contains(nullifier);
    }
}
