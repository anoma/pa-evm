// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title IKindTableRoot
/// @author Anoma Foundation, 2026
/// @notice The interface of the kind table root contract.
/// @custom:security-contact security@anoma.foundation
interface IKindTableRoot {
    /// @notice Emitted when the kind table root is set.
    /// @param kindTableRoot The Merkle root of the stored kind table.
    event KindTableRootUpdated(bytes32 indexed kindTableRoot);

    /// @notice Sets the stored kind table root. A transaction is proven against the stored kind table or against
    /// the empty kind table.
    /// @param newKindTableRoot The Merkle root of the new kind table.
    /// @dev The root changes whenever the set of supported resource kinds changes.
    function setKindTableRoot(bytes32 newKindTableRoot) external;

    /// @notice Returns the stored kind table root. A transaction is proven against the stored kind table or
    /// against the empty kind table.
    /// @return kindTableRoot The Merkle root of the current kind table.
    function getKindTableRoot() external view returns (bytes32 kindTableRoot);

    /// @notice The root of the empty kind table, under which every resource kind is derived via hash-to-curve.
    /// @return emptyKindTableRoot The Merkle root of the empty kind table: its padding leaf `SHA256.EMPTY_HASH`.
    function EMPTY_KIND_TABLE_ROOT() external view returns (bytes32 emptyKindTableRoot);
}
