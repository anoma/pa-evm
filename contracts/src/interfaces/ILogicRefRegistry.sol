// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title ILogicRefRegistry
/// @author Anoma Foundation, 2026
/// @notice The interface of the logic reference registry contract.
/// @custom:security-contact security@anoma.foundation
interface ILogicRefRegistry {
    /// @notice The status of a logic reference. `Active` restricts nothing, `Deprecated` stops the creation of the
    /// resources that carry it, and `Denied` stops their creation and their consumption.
    enum Status {
        Active,
        Deprecated,
        Denied
    }

    /// @notice A requested status change for a logic reference.
    /// @param logicRef The logic reference to update.
    /// @param status The target status, either `Deprecated` or `Denied`.
    struct StatusUpdate {
        bytes32 logicRef;
        Status status;
    }

    /// @notice Emitted when a logic reference is deprecated.
    /// @param logicRef The deprecated logic reference.
    event LogicRefDeprecated(bytes32 indexed logicRef);

    /// @notice Emitted when a logic reference is denied.
    /// @param logicRef The denied logic reference.
    event LogicRefDenied(bytes32 indexed logicRef);

    /// @notice Deprecates a logic reference so that transactions can consume its resources but cannot create them.
    /// @param logicRef The logic reference to deprecate.
    /// @dev A deprecated logic reference can still be denied, but no function makes it active again. Deprecate a logic
    /// reference only after no application creates its resources any more, since such a transaction reverts.
    function deprecateLogicRef(bytes32 logicRef) external;

    /// @notice Denies a logic reference so that no transaction consumes or creates a resource that carries it.
    /// @param logicRef The logic reference to deny.
    /// @dev No function restores a denied logic reference. An application can move its resources without consuming
    /// them, as the ERC20 forwarder migration does, and restoring it would let those resources be consumed again.
    function denyLogicRef(bytes32 logicRef) external;

    /// @notice Applies status changes in order, atomically, under the same rules as the individual setters.
    /// @param updates The logic references and their target statuses.
    /// @dev Only `Deprecated` and `Denied` are valid targets. Zero references, repeated statuses, and backwards
    /// transitions revert the entire batch. Deprecate only after applications stop creating the old resources.
    function setLogicRefStatuses(StatusUpdate[] calldata updates) external;

    /// @notice Returns the status of a logic reference.
    /// @param logicRef The logic reference to check.
    /// @return status The status of the logic reference.
    function logicRefStatus(bytes32 logicRef) external view returns (Status status);

    /// @notice Returns the number of non-active logic references.
    /// @return count The number of deprecated and denied logic references.
    function nonActiveLogicRefCount() external view returns (uint256 count);

    /// @notice Returns the non-active logic reference with the given index.
    /// @param index The index in the order the logic references became non-active.
    /// @return logicRef The logic reference at the given index.
    function nonActiveLogicRefAtIndex(uint256 index) external view returns (bytes32 logicRef);
}
