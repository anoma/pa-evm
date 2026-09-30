// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title IKindTableCommitment
/// @author Anoma Foundation, 2026
/// @notice The interface of the kind table commitment contract.
/// @custom:security-contact security@anoma.foundation
interface IKindTableCommitment {
    /// @notice Emitted when the kind table commitment is set.
    /// @param kindTableCommitment The commitment (SHA-256 hash) of the kind table transactions must be proven
    /// against.
    event KindTableCommitmentUpdated(bytes32 indexed kindTableCommitment);

    /// @notice Returns the kind table commitment that transactions must be proven against.
    /// @return kindTableCommitment The commitment (SHA-256 hash) of the current kind table.
    function getKindTableCommitment() external view returns (bytes32 kindTableCommitment);
}
