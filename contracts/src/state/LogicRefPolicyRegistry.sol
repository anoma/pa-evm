// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";

import {ILogicRefPolicyRegistry} from "../interfaces/ILogicRefPolicyRegistry.sol";

/// @title LogicRefPolicyRegistry
/// @author Anoma Foundation, 2026
/// @notice An authoritative policy per logic reference, with irreversible restrictions on resource operations.
/// @custom:security-contact security@anoma.foundation
abstract contract LogicRefPolicyRegistry is ILogicRefPolicyRegistry, Initializable {
    /// @custom:storage-location erc7201:anoma.storage.LogicRefPolicyRegistry
    struct LogicRefPolicyRegistryStorage {
        mapping(bytes32 logicRef => LogicRefPolicy policy) _policies;
        bytes32[] _restrictedLogicRefs;
    }

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.LogicRefPolicyRegistry")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _LOGIC_REF_POLICY_REGISTRY_STORAGE_SLOT =
        0x95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300;

    error ZeroLogicRefNotAllowed();
    error InvalidLogicRefPolicyTransition(bytes32 logicRef, LogicRefPolicy previousPolicy, LogicRefPolicy newPolicy);
    error ResourceWithDeniedLogicRef(bytes32 logicRef, bool consumed);

    /// @notice The constructor disabling the initializers on the implementation contract.
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @inheritdoc ILogicRefPolicyRegistry
    function setLogicRefPolicies(PolicyUpdate[] calldata updates) external override {
        _authorizeLogicRefPolicyChange();
        uint256 count = updates.length;
        for (uint256 i = 0; i < count; ++i) {
            _setLogicRefPolicy(updates[i].logicRef, updates[i].policy);
        }
    }

    /// @inheritdoc ILogicRefPolicyRegistry
    function logicRefPolicy(bytes32 logicRef) external view override returns (LogicRefPolicy policy) {
        policy = _getLogicRefPolicyRegistryStorage()._policies[logicRef];
    }

    /// @inheritdoc ILogicRefPolicyRegistry
    function restrictedLogicRefCount() external view override returns (uint256 count) {
        count = _getLogicRefPolicyRegistryStorage()._restrictedLogicRefs.length;
    }

    /// @inheritdoc ILogicRefPolicyRegistry
    function restrictedLogicRefAtIndex(uint256 index) external view override returns (bytes32 logicRef) {
        logicRef = _getLogicRefPolicyRegistryStorage()._restrictedLogicRefs[index];
    }

    /// @notice Initializes the registry. Policies default to Unrestricted and the index starts empty.
    /// @dev No storage writes are needed; this hook follows the parent initializer convention.
    // forge-lint: disable-next-line(mixed-case-function)
    function __LogicRefPolicyRegistry_init() internal onlyInitializing {}

    /// @notice Reverts unless the caller may change policies.
    function _authorizeLogicRefPolicyChange() internal virtual;

    /// @notice Adds restrictions and indexes a reference only on its first change.
    /// @param logicRef The nonzero logic reference.
    /// @param newPolicy The target policy.
    function _setLogicRefPolicy(bytes32 logicRef, LogicRefPolicy newPolicy) internal {
        require(logicRef != bytes32(0), ZeroLogicRefNotAllowed());
        LogicRefPolicyRegistryStorage storage $ = _getLogicRefPolicyRegistryStorage();
        LogicRefPolicy previousPolicy = $._policies[logicRef];
        // The enum encodes denied operations as bits. Strict inclusion rejects repeats and permission restoration.
        require(
            newPolicy != previousPolicy && (uint8(newPolicy) & uint8(previousPolicy)) == uint8(previousPolicy),
            InvalidLogicRefPolicyTransition(logicRef, previousPolicy, newPolicy)
        );
        $._policies[logicRef] = newPolicy;
        if (previousPolicy == LogicRefPolicy.Unrestricted) {
            $._restrictedLogicRefs.push(logicRef);
        }
        emit LogicRefPolicyChanged(logicRef, previousPolicy, newPolicy);
    }

    /// @notice Checks the policy for a resource operation.
    /// @param logicRef The logic reference carried by the resource.
    /// @param consumed Whether the resource is consumed rather than created.
    function _checkLogicRefNotDenied(bytes32 logicRef, bool consumed) internal view {
        LogicRefPolicy policy = _getLogicRefPolicyRegistryStorage()._policies[logicRef];
        require((uint8(policy) & (consumed ? 2 : 1)) == 0, ResourceWithDeniedLogicRef(logicRef, consumed));
    }

    /// @notice Returns the namespaced registry storage.
    /// @return registryStorage The policy mapping and enumeration index.
    function _getLogicRefPolicyRegistryStorage()
        internal
        pure
        returns (LogicRefPolicyRegistryStorage storage registryStorage)
    {
        // forge-lint: disable-next-item(inline-assembly)
        assembly {
            registryStorage.slot := _LOGIC_REF_POLICY_REGISTRY_STORAGE_SLOT
        }
    }
}
