// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {EnumerableSet} from "@openzeppelin-contracts-5.7.0/utils/structs/EnumerableSet.sol";

import {ILogicRefPolicyRegistry} from "../interfaces/ILogicRefPolicyRegistry.sol";

/// @title LogicRefPolicyRegistry
/// @author Anoma Foundation, 2026
/// @notice An authoritative policy per logic reference, with irreversible restrictions on resource operations.
/// @custom:security-contact security@anoma.foundation
abstract contract LogicRefPolicyRegistry is ILogicRefPolicyRegistry, Initializable {
    using EnumerableSet for EnumerableSet.Bytes32Set;

    /// @custom:storage-location erc7201:anoma.storage.LogicRefPolicyRegistry
    struct LogicRefPolicyRegistryStorage {
        mapping(bytes32 logicRef => LogicRefPolicy policy) _policies;
        bytes32[] _restrictedLogicRefs;
        bool _initialized;
    }

    /// @dev Read-only upgrade input. The legacy layout and namespace must remain unchanged.
    /// @custom:storage-location erc7201:anoma.storage.LogicRefDenylist
    struct LogicRefDenylistStorage {
        EnumerableSet.Bytes32Set _deniedConsumedLogicRefs;
        EnumerableSet.Bytes32Set _deniedCreatedLogicRefs;
    }

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.LogicRefPolicyRegistry")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _LOGIC_REF_POLICY_REGISTRY_STORAGE_SLOT =
        0x95de9d243f5e9816b043bbf387b59f6a6f6ae8729bedcdcbd709d11c3a33e300;
    // The temporary migrational adapter excludes the importer; this slot is used by the plain adapter.
    // slither-disable-next-line unused-state
    bytes32 internal constant _LOGIC_REF_DENYLIST_STORAGE_SLOT =
        0x4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900;

    error ZeroLogicRefNotAllowed();
    error InvalidLogicRefPolicyTransition(bytes32 logicRef, LogicRefPolicy previousPolicy, LogicRefPolicy newPolicy);
    error ResourceWithDeniedLogicRef(bytes32 logicRef, bool consumed);
    error LogicRefPoliciesNotInitialized();

    /// @notice The constructor disabling the initializers on the implementation contract.
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @inheritdoc ILogicRefPolicyRegistry
    function setLogicRefPolicies(PolicyUpdate[] calldata updates) external override {
        _authorizeLogicRefPolicyChange();
        _requireLogicRefPoliciesInitialized();
        uint256 count = updates.length;
        for (uint256 i = 0; i < count; ++i) {
            _setLogicRefPolicy(updates[i].logicRef, updates[i].policy);
        }
    }

    /// @inheritdoc ILogicRefPolicyRegistry
    function initializeLogicRefPolicies() external virtual override reinitializer(2) {
        _authorizeLogicRefPolicyChange();
        LogicRefPolicyRegistryStorage storage $ = _getLogicRefPolicyRegistryStorage();
        if ($._initialized) return;

        LogicRefDenylistStorage storage legacy;
        // forge-lint: disable-next-item(inline-assembly)
        assembly {
            legacy.slot := _LOGIC_REF_DENYLIST_STORAGE_SLOT
        }
        uint256 count = legacy._deniedConsumedLogicRefs.length();
        for (uint256 i = 0; i < count; ++i) {
            bytes32 logicRef = legacy._deniedConsumedLogicRefs.at(i);
            _setLogicRefPolicy(
                logicRef,
                legacy._deniedCreatedLogicRefs.contains(logicRef)
                    ? LogicRefPolicy.FullyDenied
                    : LogicRefPolicy.ConsumptionDenied
            );
        }
        count = legacy._deniedCreatedLogicRefs.length();
        for (uint256 i = 0; i < count; ++i) {
            bytes32 logicRef = legacy._deniedCreatedLogicRefs.at(i);
            if ($._policies[logicRef] == LogicRefPolicy.Unrestricted) {
                _setLogicRefPolicy(logicRef, LogicRefPolicy.CreationDenied);
            }
        }
        $._initialized = true;
    }

    /// @inheritdoc ILogicRefPolicyRegistry
    function logicRefPolicy(bytes32 logicRef) external view override returns (LogicRefPolicy policy) {
        _requireLogicRefPoliciesInitialized();
        policy = _getLogicRefPolicyRegistryStorage()._policies[logicRef];
    }

    /// @inheritdoc ILogicRefPolicyRegistry
    function restrictedLogicRefCount() external view override returns (uint256 count) {
        _requireLogicRefPoliciesInitialized();
        count = _getLogicRefPolicyRegistryStorage()._restrictedLogicRefs.length;
    }

    /// @inheritdoc ILogicRefPolicyRegistry
    function restrictedLogicRefAtIndex(uint256 index) external view override returns (bytes32 logicRef) {
        _requireLogicRefPoliciesInitialized();
        logicRef = _getLogicRefPolicyRegistryStorage()._restrictedLogicRefs[index];
    }

    /// @notice Initializes an empty registry for a fresh deployment.
    // forge-lint: disable-next-line(mixed-case-function)
    function __LogicRefPolicyRegistry_init() internal onlyInitializing {
        _getLogicRefPolicyRegistryStorage()._initialized = true;
    }

    /// @notice Reverts unless the caller may change policies or import legacy restrictions.
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

    /// @notice Checks the policy for a resource operation. The caller must first check registry initialization.
    /// @param logicRef The logic reference carried by the resource.
    /// @param consumed Whether the resource is consumed rather than created.
    function _checkLogicRefNotDenied(bytes32 logicRef, bool consumed) internal view {
        LogicRefPolicy policy = _getLogicRefPolicyRegistryStorage()._policies[logicRef];
        require((uint8(policy) & (consumed ? 2 : 1)) == 0, ResourceWithDeniedLogicRef(logicRef, consumed));
    }

    /// @notice Prevents a missed upgrade initializer from exposing legacy restrictions as unrestricted.
    function _requireLogicRefPoliciesInitialized() internal view {
        require(_getLogicRefPolicyRegistryStorage()._initialized, LogicRefPoliciesNotInitialized());
    }

    /// @notice Returns the namespaced registry storage.
    /// @return registryStorage The policy mapping, enumeration index and initialization flag.
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
