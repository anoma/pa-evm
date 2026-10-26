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

    /// @notice Returns the number of commitments that have been added to the tree.
    /// @return count The number of commitments in the tree.
    function commitmentCount() external view returns (uint256 count);

    /// @notice Returns the commitment tree depth.
    /// @return depth The depth of the tree.
    function commitmentTreeDepth() external view returns (uint8 depth);

    /// @notice Computes the capacity of the tree based on the current tree depth.
    /// @return capacity The computed tree capacity.
    function commitmentTreeCapacity() external view returns (uint256 capacity);

    /// @notice Returns the left siblings of the commitment tree: for each level below the root, the root of the last
    /// complete left subtree on that level. The tree keeps them to add commitments without storing the earlier ones.
    /// Only the levels where the commitment count has a one bit are meaningful; the others hold stale values.
    /// @return leftSiblings The left siblings, from the leaf level up.
    function commitmentTreeLeftSiblings() external view returns (bytes32[] memory leftSiblings);

    /// @notice Returns the empty subtree roots of the commitment tree: for each level up to the root, the root of an
    /// empty subtree of that height.
    /// @return emptySubtreeRoots The empty subtree roots, from the leaf level up.
    function commitmentTreeEmptySubtreeRoots() external view returns (bytes32[] memory emptySubtreeRoots);

    /// @notice Returns the latest commitment tree root: the root of the tree with all commitments added so far.
    /// @return root The latest commitment tree root.
    function latestCommitmentTreeRoot() external view returns (bytes32 root);

    /// @notice Returns whether a consumed resource can reference a commitment tree root.
    /// @param root The commitment tree root that a consumed resource references.
    /// @return isHistorical Whether the contract stored the root as a historical root.
    function isCommitmentTreeRootHistorical(bytes32 root) external view returns (bool isHistorical);
}
