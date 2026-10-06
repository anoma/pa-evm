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

    /// @notice Emitted when a logic reference is deprecated.
    /// @param logicRef The deprecated logic reference.
    event LogicRefDeprecated(bytes32 indexed logicRef);

    /// @notice Emitted when a logic reference is denied.
    /// @param logicRef The denied logic reference.
    event LogicRefDenied(bytes32 indexed logicRef);

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
