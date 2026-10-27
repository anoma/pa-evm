// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title IMigrational
/// @author Anoma Foundation, 2026
/// @notice The interface of the protocol adapter variant that takes over the state of a stopped legacy
/// protocol adapter.
/// @custom:security-contact security@anoma.foundation
interface IMigrational {
    /// @notice Emitted when the commitment tree state has been copied in.
    /// @param root The root of the copied tree, now the latest historical root.
    /// @param leafCount The number of commitments in the copied tree.
    event CommitmentTreeMigrated(bytes32 indexed root, uint256 leafCount);

    /// @notice Emitted when a batch of nullifiers has been migrated.
    /// @param start The index of the first nullifier of the batch in the legacy protocol adapter.
    /// @param count The number of nullifiers in the batch.
    event NullifierBatchMigrated(uint256 start, uint256 count);

    /// @notice Emitted when the migration is completed.
    event MigrationFinalized();

    /// @notice Copies the commitment tree of the legacy protocol adapter in. Allowed once, while paused, and
    /// only while the legacy protocol adapter is stopped.
    /// @param leftSiblings The left siblings that the legacy protocol adapter stores but does not return, one per level
    /// below the root. The function checks them against the latest root of the legacy commitment tree.
    function migrateCommitmentTree(bytes32[] calldata leftSiblings) external;

    /// @notice Copies `count` nullifiers of the legacy protocol adapter in, from index `start` on. Allowed
    /// while paused, and only while the legacy protocol adapter is stopped. Call it until every nullifier is in.
    /// @param start The index of the first nullifier to copy in the legacy protocol adapter. It must equal the
    /// number of nullifiers copied so far.
    /// @param count The number of nullifiers to copy in this batch.
    function migrateNullifierSet(uint256 start, uint256 count) external;

    /// @notice Returns the legacy protocol adapter this contract copies its state from.
    /// @return legacyProtocolAdapter The legacy protocol adapter.
    function getLegacyProtocolAdapter() external view returns (address legacyProtocolAdapter);
}
