// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title ILegacyCommitmentTree
/// @author Anoma Foundation, 2026
/// @notice The functions of the commitment tree of the legacy protocol adapter that the state migration reads.
/// @custom:security-contact security@anoma.foundation
interface ILegacyCommitmentTree {
    /// @notice Returns the number of commitments that have been added to the tree.
    /// @return count The number of commitments in the tree.
    function commitmentCount() external view returns (uint256 count);

    /// @notice Returns the commitment tree depth.
    /// @return depth The depth of the tree.
    function commitmentTreeDepth() external view returns (uint8 depth);

    /// @notice Returns the latest commitment tree root.
    /// @return root The latest commitment tree root.
    function latestCommitmentTreeRoot() external view returns (bytes32 root);
}
