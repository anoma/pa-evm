// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title ILogicRefDenylist
/// @author Anoma Foundation, 2026
/// @notice The interface of the logic reference denylist contract.
/// @custom:security-contact security@anoma.foundation
interface ILogicRefDenylist {
    /// @notice Emitted when a logic reference is added to the denylist.
    /// @param logicRef The denied logic reference.
    event LogicRefDenied(bytes32 indexed logicRef);

    /// @notice Returns whether the denylist contains a given logic reference or not.
    /// @param logicRef The logic reference to check.
    /// @return isDenied Whether the logic reference is denied or not.
    function isLogicRefDenied(bytes32 logicRef) external view returns (bool isDenied);

    /// @notice Returns the number of logic references in the denylist.
    /// @return count The number of denied logic references.
    function deniedLogicRefCount() external view returns (uint256 count);

    /// @notice Returns the denied logic reference with the given index.
    /// @param index The index, in the order the logic references were denied.
    /// @return logicRef The logic reference at the given index.
    function deniedLogicRefAtIndex(uint256 index) external view returns (bytes32 logicRef);
}
