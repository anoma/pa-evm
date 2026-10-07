// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title ILogicRefPolicyRegistry
/// @author Anoma Foundation, 2026
/// @notice Policies restricting creation and consumption of resources carrying a logic reference.
/// @custom:security-contact security@anoma.foundation
interface ILogicRefPolicyRegistry {
    /// @notice The restrictions on resource creation and consumption.
    enum LogicRefPolicy {
        Unrestricted,
        CreationDenied,
        ConsumptionDenied,
        FullyDenied
    }

    /// @notice An explicit target policy for a logic reference.
    /// @param logicRef The nonzero logic reference.
    /// @param policy The target policy, which must strictly add restrictions.
    struct PolicyUpdate {
        bytes32 logicRef;
        LogicRefPolicy policy;
    }

    /// @notice Emitted once per policy change.
    /// @param logicRef The affected logic reference.
    /// @param previousPolicy The previous registry policy.
    /// @param newPolicy The new registry policy.
    event LogicRefPolicyChanged(bytes32 indexed logicRef, LogicRefPolicy previousPolicy, LogicRefPolicy newPolicy);

    /// @notice Applies an ordered, atomic batch of policy changes. An empty batch is a no-op.
    /// @param updates The target policies. Repeated references may occur if each transition is valid.
    /// @dev Only strictly stronger restrictions are allowed; a repeated policy or restoring either permission
    /// reverts the whole batch. Restrictions are irreversible: applications may move backing assets without consuming
    /// the old resources. Creation denial also rejects ephemeral resources, so consumption remaining allowed does
    /// not by itself guarantee a working withdrawal path.
    function setLogicRefPolicies(PolicyUpdate[] calldata updates) external;

    /// @notice Returns the policy for a logic reference; unknown references are unrestricted.
    /// @param logicRef The logic reference to query.
    /// @return policy The current policy.
    function logicRefPolicy(bytes32 logicRef) external view returns (LogicRefPolicy policy);

    /// @notice Returns the number of restricted logic references.
    /// @return count The number of references in the enumeration index.
    function restrictedLogicRefCount() external view returns (uint256 count);

    /// @notice Returns a restricted reference in order of its first restriction.
    /// @param index The index to query.
    /// @return logicRef The restricted logic reference.
    function restrictedLogicRefAtIndex(uint256 index) external view returns (bytes32 logicRef);
}
