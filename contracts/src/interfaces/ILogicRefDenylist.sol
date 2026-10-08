// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title ILogicRefDenylist
/// @author Anoma Foundation, 2026
/// @notice The interface of the logic reference denylist contract, which holds one denylist for the logic references
/// of consumed resources and one for those of created resources.
/// @custom:security-contact security@anoma.foundation
interface ILogicRefDenylist {
    /// @notice A logic reference to add to a denylist.
    /// @param logicRef The logic reference to deny.
    /// @param consumed `true` for the denylist for consumed resources, `false` for the one for created resources.
    struct DeniedLogicRef {
        bytes32 logicRef;
        bool consumed;
    }

    /// @notice Emitted when a logic reference is added to a denylist.
    /// @param logicRef The denied logic reference.
    /// @param consumed `true` for the denylist for consumed resources, `false` for the one for created resources.
    event LogicRefDenied(bytes32 indexed logicRef, bool consumed);

    /// @notice Adds logic references to the denylists. To deprecate a logic reference, add it to the denylist for
    /// created resources: transactions still consume its resources. To deny it, add it to both denylists.
    /// @param logicRefs The logic references to deny, each with the denylist to add it to.
    /// @dev The call reverts if a logic reference is zero or already on its denylist. No function removes an entry. An
    /// application can move the resources of a denied logic reference without consuming them, as the ERC20 forwarder
    /// migration does, and a removed entry would let them be consumed again. Deprecate a logic reference only after no
    /// application creates its resources any more, since such a transaction reverts.
    function denyLogicRefs(DeniedLogicRef[] calldata logicRefs) external;

    /// @notice Returns whether a denylist contains a logic reference.
    /// @param logicRef The logic reference to check.
    /// @param consumed `true` for the denylist for consumed resources, `false` for the one for created resources.
    /// @return isDenied Whether the denylist contains the logic reference.
    function isLogicRefDenied(bytes32 logicRef, bool consumed) external view returns (bool isDenied);
}
