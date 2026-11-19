// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title ICommitmentTree
/// @author Anoma Foundation, 2025
/// @notice The interface of the commitment tree contract.
/// @custom:security-contact security@anoma.foundation
interface ICommitmentTree {
    /// @notice Emitted when a commitment tree root is added to the set of historical roots.
    /// @param root The commitment tree root being stored.
    event CommitmentTreeRootAdded(bytes32 root);

    /// @notice Returns the number of commitments in the commitment tree.
    /// @return count The number of commitments, which is also the index of the next commitment.
    function commitmentCount() external view returns (uint256 count);

    /// @notice Returns the depth of the commitment tree.
    /// @return depth The number of levels below the root.
    function commitmentTreeDepth() external view returns (uint8 depth);

    /// @notice Computes the capacity of the commitment tree: the number of commitments that fill it at its depth.
    /// @return capacity Two to the power of the depth.
    function commitmentTreeCapacity() external view returns (uint256 capacity);

    /// @notice Returns the left siblings of the commitment tree, one per level below the root.
    /// @return leftSiblings The left siblings, from the leaf level up.
    function commitmentTreeLeftSiblings() external view returns (bytes32[] memory leftSiblings);

    /// @notice Returns the empty subtree roots of the commitment tree, one per level up to the root.
    /// @return emptySubtreeRoots The roots of subtrees without commitments, from the leaf level up.
    function commitmentTreeEmptySubtreeRoots() external view returns (bytes32[] memory emptySubtreeRoots);

    /// @notice Returns the latest commitment tree root: the root of the tree with all commitments added so far.
    /// @return root The latest commitment tree root.
    function latestCommitmentTreeRoot() external view returns (bytes32 root);

    /// @notice Returns whether a consumed resource can reference a commitment tree root.
    /// @param root The commitment tree root that a consumed resource references.
    /// @return isHistorical Whether the contract stored the root as a historical root.
    function isCommitmentTreeRootHistorical(bytes32 root) external view returns (bool isHistorical);
}
