// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title IKindTreeRoot
/// @author Anoma Foundation, 2026
/// @notice The interface of the kind tree root contract.
/// @custom:security-contact security@anoma.foundation
interface IKindTreeRoot {
    /// @notice Emitted when the kind tree root is set.
    /// @dev The owner can set the root from inside a forwarder call, so this event can precede the
    /// `TransactionExecuted` event of a transaction proven against the previous root. A transaction's kind tree root
    /// is `Transaction.kindTreeRoot`, not the root last emitted before it.
    /// @param kindTreeRoot The Merkle root of the stored kind tree.
    event KindTreeRootUpdated(bytes32 indexed kindTreeRoot);

    /// @notice Sets the stored kind tree root. A transaction is proven against the stored kind tree or against
    /// the empty kind tree.
    /// @param newKindTreeRoot The Merkle root of the new kind tree.
    function setKindTreeRoot(bytes32 newKindTreeRoot) external;

    /// @notice Returns the stored kind tree root. A transaction is proven against the stored kind tree or
    /// against the empty kind tree.
    /// @return kindTreeRoot The Merkle root of the current kind tree.
    function getKindTreeRoot() external view returns (bytes32 kindTreeRoot);

    /// @notice The root of the empty kind tree. In a transaction proven against the empty kind tree, a resource
    /// balances only against resources of the same kind.
    /// @return emptyKindTreeRoot The Merkle root of the empty kind tree: its padding leaf `SHA256.EMPTY_HASH`.
    function EMPTY_KIND_TREE_ROOT() external view returns (bytes32 emptyKindTreeRoot);
}
