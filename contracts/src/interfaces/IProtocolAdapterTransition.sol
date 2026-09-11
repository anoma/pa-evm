// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title IProtocolAdapterTransition
/// @author Anoma Foundation, 2026
/// @notice The interface of the protocol adapter variant that takes over the state of a stopped v1 protocol adapter.
/// @custom:security-contact security@anoma.foundation
interface IProtocolAdapterTransition {
    /// @notice Emitted when the commitment tree state has been copied in.
    /// @param root The root of the copied tree, now the latest historical root.
    /// @param leafCount The number of commitments in the copied tree.
    event CommitmentTreeSeeded(bytes32 indexed root, uint256 leafCount);

    /// @notice Emitted when a batch of nullifiers has been copied in.
    /// @param start The index of the first nullifier of the batch, in both protocol adapters.
    /// @param count The number of nullifiers in the batch.
    event NullifierSetSeeded(uint256 start, uint256 count);

    /// @notice Copies the commitment tree of the v1 protocol adapter in. Allowed once, while paused, and only while
    /// the v1 protocol adapter is stopped.
    /// @param sides The v1 tree's stored sides, one per level. This is the one part of the tree that v1 does not
    /// expose through a getter, so it is passed in and checked against v1's root.
    /// @dev The leaf count and the root are read from the v1 protocol adapter, and the zero hashes are recomputed
    /// from the depth, so `sides` is the only value a caller supplies. The historical roots then are the empty-tree
    /// root at index 0 and v1's latest root at index 1. The older roots of v1 are dropped. The seeded root is emitted as
    /// `CommitmentTreeRootAdded`, like every root added by a transaction, so an indexer sees it as one more root.
    function seedCommitmentTree(bytes32[] calldata sides) external;

    /// @notice Copies the next `count` nullifiers of the v1 protocol adapter in. Allowed while paused, and only while
    /// the v1 protocol adapter is stopped. Call it until every nullifier is in.
    /// @param count The number of nullifiers to copy in this batch.
    /// @dev The batch starts at the index this protocol adapter has reached, so a batch can neither be skipped nor
    /// repeated. Every nullifier is read from v1 at that index, and each one is required to land at the same index
    /// here.
    function seedNullifierSet(uint256 count) external;

    /// @notice Returns the v1 protocol adapter this contract copies its state from.
    /// @return protocolAdapterV1 The v1 protocol adapter.
    function getProtocolAdapterV1() external view returns (address protocolAdapterV1);
}
