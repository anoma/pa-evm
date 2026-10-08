// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";

import {ICommitmentTree} from "../interfaces/ICommitmentTree.sol";
import {MerkleTree} from "../libs/MerkleTree.sol";

/// @title CommitmentTree
/// @author Anoma Foundation, 2025
/// @notice A commitment tree being inherited by the protocol adapter.
/// @dev The tree is a modified version of OpenZeppelin's `MerkleTree`, and the set of historical roots neither counts
/// nor lists its roots.
/// @custom:security-contact security@anoma.foundation
abstract contract CommitmentTree is ICommitmentTree, Initializable {
    using MerkleTree for MerkleTree.Tree;

    /// @custom:storage-location erc7201:anoma.storage.CommitmentTree
    struct CommitmentTreeStorage {
        MerkleTree.Tree _merkleTree;
        mapping(bytes32 root => bool isHistorical) _historicalRoots;
    }

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.CommitmentTree")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _COMMITMENT_TREE_STORAGE_SLOT =
        0x762a46a11c460b9bcb2bb98651da03b192a02e2a33ab26da9bf9ed53826bc900;

    error NonExistingRoot(bytes32 root);
    error PreExistingRoot(bytes32 root);

    /// @notice The constructor disabling the initializers on the implementation contract.
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @inheritdoc ICommitmentTree
    function commitmentCount() external view override returns (uint256 count) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        count = $._merkleTree.leafCount();
    }

    /// @inheritdoc ICommitmentTree
    function commitmentTreeDepth() external view override returns (uint8 depth) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        depth = $._merkleTree.depth();
    }

    /// @inheritdoc ICommitmentTree
    function commitmentTreeCapacity() external view override returns (uint256 capacity) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        capacity = $._merkleTree.capacity();
    }

    /// @inheritdoc ICommitmentTree
    function commitmentTreeSides() external view override returns (bytes32[] memory sides) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        sides = $._merkleTree.sides();
    }

    /// @inheritdoc ICommitmentTree
    function commitmentTreeZeros() external view override returns (bytes32[] memory zeros) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        zeros = $._merkleTree.zeros();
    }

    /// @inheritdoc ICommitmentTree
    function isCommitmentTreeRootHistorical(bytes32 root) external view override returns (bool isHistorical) {
        isHistorical = _isCommitmentTreeRootHistorical(root);
    }

    /// @inheritdoc ICommitmentTree
    function latestCommitmentTreeRoot() external view override returns (bytes32 root) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        root = $._merkleTree.currentRoot();
    }

    /// @notice Initializes the commitment tree by setting up the underlying Merkle tree and storing the initial root.
    // forge-lint: disable-next-line(mixed-case-function)
    function __CommitmentTree_init() internal onlyInitializing {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        bytes32 initialRoot = $._merkleTree.setup();

        $._historicalRoots[initialRoot] = true;

        emit CommitmentTreeRootAdded({root: initialRoot});
    }

    /// @notice Adds a commitment to the accumulator and returns the new root.
    /// @param commitment The commitment to add.
    /// @return newRoot The resulting new root.
    function _addCommitment(bytes32 commitment) internal returns (bytes32 newRoot) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        uint256 index;
        (index, newRoot) = $._merkleTree.push(commitment);
    }

    /// @notice Adds a root to the set of historical roots and emits the `CommitmentTreeRootAdded` event.
    /// @param root The root to store.
    function _addCommitmentTreeRoot(bytes32 root) internal {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        require(!$._historicalRoots[root], PreExistingRoot(root));
        $._historicalRoots[root] = true;

        emit CommitmentTreeRootAdded(root);
    }

    /// @notice Returns whether the set of historical roots contains a commitment tree root.
    /// @param root The commitment tree root to look up.
    /// @return isHistorical Whether a consumed resource can reference the root.
    function _isCommitmentTreeRootHistorical(bytes32 root) internal view returns (bool isHistorical) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        isHistorical = $._historicalRoots[root];
    }

    /// @notice Returns the storage from the commitment tree storage location.
    /// @return commitmentTreeStorage The data associated with the commitment tree storage.
    function _getCommitmentTreeStorage() internal pure returns (CommitmentTreeStorage storage commitmentTreeStorage) {
        // forge-lint: disable-next-item(inline-assembly)
        assembly {
            commitmentTreeStorage.slot := _COMMITMENT_TREE_STORAGE_SLOT
        }
    }
}
