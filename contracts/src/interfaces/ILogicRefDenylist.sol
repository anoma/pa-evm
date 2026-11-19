// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title ILogicRefDenylist
/// @author Anoma Foundation, 2026
/// @notice The interface of a contract maintaining logic reference denylists for consumed and created resources.
/// @custom:security-contact security@anoma.foundation
interface ILogicRefDenylist {
    /// @notice A logic reference to add to a denylist.
    /// @param logicRef The logic reference to deny.
    /// @param consumed Whether the denylist is the one for consumed resources.
    struct DeniedLogicRef {
        bytes32 logicRef;
        bool consumed;
    }

    /// @notice Emitted when a logic reference is added to a denylist.
    /// @param logicRef The denied logic reference.
    /// @param consumed Whether the denylist is the one for consumed resources.
    event LogicRefDenied(bytes32 indexed logicRef, bool consumed);

    /// @notice Adds logic references to the denylists. To deprecate a logic reference, add it to the denylist for
    /// created resources: transactions still consume its resources. To deny it, add it to both denylists.
    /// @param logicRefs The logic references to deny, each with the denylist to add it to.
    function denyLogicRefs(DeniedLogicRef[] calldata logicRefs) external;

    /// @notice Returns whether a denylist contains a given logic reference or not.
    /// @param logicRef The logic reference to check.
    /// @param consumed Whether the denylist is the one for consumed resources.
    /// @return isDenied Whether the denylist contains the logic reference or not.
    function isLogicRefDenied(bytes32 logicRef, bool consumed) external view returns (bool isDenied);
}
