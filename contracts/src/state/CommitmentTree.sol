// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.6.1/proxy/utils/Initializable.sol";

import {EnumerableSet} from "@openzeppelin-contracts-5.6.1/utils/structs/EnumerableSet.sol";

import {ICommitmentTree} from "../interfaces/ICommitmentTree.sol";
import {MerkleTree} from "../libs/MerkleTree.sol";

/// @title CommitmentTree
/// @author Anoma Foundation, 2025
/// @notice A commitment tree being inherited by the protocol adapter.
/// @dev The contract is based on a modified version of OZ's `MerkleTree` implementation and and the unchanged OZ
/// `EnumerableSet` implementation.
/// @custom:security-contact security@anoma.foundation
abstract contract CommitmentTree is ICommitmentTree, Initializable {
    using MerkleTree for MerkleTree.Tree;
    using EnumerableSet for EnumerableSet.Bytes32Set;

    /// @custom:storage-location erc7201:anoma.storage.CommitmentTree
    struct CommitmentTreeStorage {
        MerkleTree.Tree _merkleTree;
        EnumerableSet.Bytes32Set _roots;
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

    /// @notice Initializes the Commitment tree contract.
    function initialize() external initializer /* solhint-disable-line comprehensive-interface */  {
        __CommitmentTree_init();
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
    function isCommitmentTreeRootContained(bytes32 root) external view override returns (bool isContained) {
        isContained = _isCommitmentTreeRootContained(root);
    }

    /// @inheritdoc ICommitmentTree
    function commitmentTreeRootCount() external view override returns (uint256 count) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        count = $._roots.length();
    }

    /// @inheritdoc ICommitmentTree
    function commitmentTreeRootAtIndex(uint256 index) external view override returns (bytes32 root) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        root = $._roots.at(index);
    }

    /// @inheritdoc ICommitmentTree
    function latestCommitmentTreeRoot() external view override returns (bytes32 root) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        root = $._roots.at($._roots.length() - 1);
    }

    /// @notice Initializes the commitment tree by setting up the underlying Merkle tree and storing the initial root.
    // solhint-disable-next-line func-name-mixedcase
    function __CommitmentTree_init() internal onlyInitializing {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        bytes32 initialRoot = $._merkleTree.setup();

        // slither-disable-next-line unused-return
        $._roots.add(initialRoot);

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

        require($._roots.add(root), PreExistingRoot(root));
        emit CommitmentTreeRootAdded(root);
    }

    /// @notice Checks if a commitment tree root is contained in the set of historical roots.
    /// @param root The root to check.
    /// @return isContained Whether the root exists or not.
    function _isCommitmentTreeRootContained(bytes32 root) internal view returns (bool isContained) {
        CommitmentTreeStorage storage $ = _getCommitmentTreeStorage();

        isContained = $._roots.contains(root);
    }

    /// @notice Returns the storage from the commitment tree storage location.
    /// @return commitmentTreeStorage The data associated with the commitment tree storage.
    function _getCommitmentTreeStorage() internal pure returns (CommitmentTreeStorage storage commitmentTreeStorage) {
        /* solhint-disable no-inline-assembly */

        // slither-disable-next-line assembly
        assembly {
            commitmentTreeStorage.slot := _COMMITMENT_TREE_STORAGE_SLOT
        }

        /* solhint-enable no-inline-assembly */
    }
}
