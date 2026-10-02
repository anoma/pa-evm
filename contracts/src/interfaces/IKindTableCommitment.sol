// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title IKindTableCommitment
/// @author Anoma Foundation, 2026
/// @notice The interface of the kind table commitment contract.
/// @custom:security-contact security@anoma.foundation
interface IKindTableCommitment {
    /// @notice Emitted when the kind table commitment is set.
    /// @param kindTableCommitment The commitment (SHA-256 hash) of the stored kind table.
    event KindTableCommitmentUpdated(bytes32 indexed kindTableCommitment);

    /// @notice Returns the stored kind table commitment. A transaction is proven against the stored kind table or
    /// against the empty kind table.
    /// @return kindTableCommitment The commitment (SHA-256 hash) of the current kind table.
    function getKindTableCommitment() external view returns (bytes32 kindTableCommitment);

    /// @notice The commitment of the empty kind table, under which every resource kind is derived via hash-to-curve.
    /// @return emptyKindTableCommitment The SHA-256 hash of zero bytes of table content.
    function EMPTY_KIND_TABLE_COMMITMENT() external view returns (bytes32 emptyKindTableCommitment);
}
