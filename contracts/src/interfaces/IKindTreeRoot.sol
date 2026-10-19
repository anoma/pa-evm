// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title IKindTreeRoot
/// @author Anoma Foundation, 2026
/// @notice The interface of the kind tree root contract.
/// @custom:security-contact security@anoma.foundation
interface IKindTreeRoot {
    /// @notice Emitted when the kind tree root is set.
    /// @param kindTreeRoot The Merkle root of the stored kind table.
    event KindTreeRootUpdated(bytes32 indexed kindTreeRoot);

    /// @notice Sets the stored kind tree root. A transaction is proven against the stored kind table or against
    /// the empty kind table.
    /// @param newKindTreeRoot The Merkle root of the new kind table.
    /// @dev The root changes whenever the set of supported resource kinds changes.
    function setKindTreeRoot(bytes32 newKindTreeRoot) external;

    /// @notice Returns the stored kind tree root. A transaction is proven against the stored kind table or
    /// against the empty kind table.
    /// @return kindTreeRoot The Merkle root of the current kind table.
    function getKindTreeRoot() external view returns (bytes32 kindTreeRoot);

    /// @notice The root of the empty kind tree, under which every resource kind is derived via hash-to-curve.
    /// @return emptyKindTreeRoot The Merkle root of the empty kind tree: its padding leaf `SHA256.EMPTY_HASH`.
    function EMPTY_KIND_TREE_ROOT() external view returns (bytes32 emptyKindTreeRoot);
}
