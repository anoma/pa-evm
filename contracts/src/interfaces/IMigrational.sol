// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title IMigrational
/// @author Anoma Foundation, 2026
/// @notice The interface of the protocol adapter variant that takes over the state of a stopped immutable protocol
/// adapter.
/// @custom:security-contact security@anoma.foundation
interface IMigrational {
    /// @notice Emitted when the commitment tree state has been copied in.
    /// @param root The root of the copied tree, now the latest historical root.
    /// @param leafCount The number of commitments in the copied tree.
    event CommitmentTreeMigrated(bytes32 indexed root, uint256 leafCount);

    /// @notice Emitted when a batch of nullifiers has been migrated.
    /// @param start The index of the first nullifier of the batch in the immutable protocol adapter.
    /// @param count The number of nullifiers in the batch.
    event NullifierBatchMigrated(uint256 start, uint256 count);

    /// @notice Emitted when the migration is completed.
    event MigrationFinalized();

    /// @notice Copies the commitment tree of the immutable protocol adapter in. Allowed once, while paused, and only
    /// while the immutable protocol adapter is stopped.
    /// @param sides The stored sides of the tree of the immutable protocol adapter, one per level. This is the one part
    /// of the tree that it does not expose through a getter, so it is passed in and checked against its root.
    function migrateCommitmentTree(bytes32[] calldata sides) external;

    /// @notice Copies `count` nullifiers of the immutable protocol adapter in, from index `start` on. Allowed while
    /// paused, and only while the immutable protocol adapter is stopped. Call it until every nullifier is in.
    /// @param start The index of the first nullifier to copy in the immutable protocol adapter. It must equal the
    /// number of nullifiers copied so far.
    /// @param count The number of nullifiers to copy in this batch.
    function migrateNullifierSet(uint256 start, uint256 count) external;

    /// @notice Returns the immutable protocol adapter this contract copies its state from.
    /// @return immutableProtocolAdapter The immutable protocol adapter.
    function getImmutableProtocolAdapter() external view returns (address immutableProtocolAdapter);
}
