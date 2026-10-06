// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {EnumerableSet} from "@openzeppelin-contracts-5.7.0/utils/structs/EnumerableSet.sol";

import {ILogicRefRegistry} from "../interfaces/ILogicRefRegistry.sol";

/// @title LogicRefRegistry
/// @author Anoma Foundation, 2026
/// @notice The logic reference registry inherited by the protocol adapter.
/// @dev A status only becomes stricter, from active to deprecated to denied. No function removes a listed logic
/// reference.
/// @custom:security-contact security@anoma.foundation
abstract contract LogicRefRegistry is ILogicRefRegistry, Initializable {
    using EnumerableSet for EnumerableSet.Bytes32Set;

    // Retain the original namespace so this rename preserves existing state.
    /// @custom:storage-location erc7201:anoma.storage.LogicRefStatuses
    struct LogicRefRegistryStorage {
        EnumerableSet.Bytes32Set _nonActiveLogicRefs;
        mapping(bytes32 logicRef => Status status) _logicRefStatus;
    }

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.LogicRefStatuses")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _LOGIC_REF_REGISTRY_STORAGE_SLOT =
        0x6c0e57a925c567fdc507a8edfd821e8dd3a8b4619b6270796cf61f3ebf8cf500;

    error ZeroLogicRefNotAllowed();
    error LogicRefAlreadyDeprecated(bytes32 logicRef);
    error LogicRefAlreadyDenied(bytes32 logicRef);
    error DeprecatedLogicRef(bytes32 logicRef);
    error DeniedLogicRef(bytes32 logicRef);

    /// @notice The constructor disabling the initializers on the implementation contract.
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @inheritdoc ILogicRefRegistry
    function logicRefStatus(bytes32 logicRef) public view override returns (Status status) {
        LogicRefRegistryStorage storage $ = _logicRefRegistryStorage();

        status = $._logicRefStatus[logicRef];
    }

    /// @inheritdoc ILogicRefRegistry
    function nonActiveLogicRefCount() external view override returns (uint256 count) {
        LogicRefRegistryStorage storage $ = _logicRefRegistryStorage();

        count = $._nonActiveLogicRefs.length();
    }

    /// @inheritdoc ILogicRefRegistry
    function nonActiveLogicRefAtIndex(uint256 index) external view override returns (bytes32 logicRef) {
        LogicRefRegistryStorage storage $ = _logicRefRegistryStorage();

        logicRef = $._nonActiveLogicRefs.at(index);
    }

    /// @notice Initializes the LogicRefRegistry contract.
    /// @dev Every logic reference starts active, so the contract requires no setup. The function exists for
    /// consistency with the OpenZeppelin initializer convention.
    // forge-lint: disable-next-line(mixed-case-function)
    function __LogicRefRegistry_init() internal onlyInitializing {}

    /// @notice Deprecates an active logic reference and emits the `LogicRefDeprecated` event.
    /// @param logicRef The logic reference to deprecate.
    function _deprecateLogicRef(bytes32 logicRef) internal {
        require(logicRef != bytes32(0), ZeroLogicRefNotAllowed());

        Status status = logicRefStatus(logicRef);
        require(status != Status.Deprecated, LogicRefAlreadyDeprecated(logicRef));
        require(status != Status.Denied, LogicRefAlreadyDenied(logicRef));
        assert(status == Status.Active);

        LogicRefRegistryStorage storage $ = _logicRefRegistryStorage();
        bool added = $._nonActiveLogicRefs.add(logicRef);
        assert(added);
        $._logicRefStatus[logicRef] = Status.Deprecated;

        emit LogicRefDeprecated({logicRef: logicRef});
    }

    /// @notice Denies an active or deprecated logic reference and emits the `LogicRefDenied` event.
    /// @param logicRef The logic reference to deny.
    function _denyLogicRef(bytes32 logicRef) internal {
        require(logicRef != bytes32(0), ZeroLogicRefNotAllowed());

        Status status = logicRefStatus(logicRef);
        require(status != Status.Denied, LogicRefAlreadyDenied(logicRef));

        LogicRefRegistryStorage storage $ = _logicRefRegistryStorage();
        if (status == Status.Active) {
            bool added = $._nonActiveLogicRefs.add(logicRef);
            assert(added);
        }
        $._logicRefStatus[logicRef] = Status.Denied;

        emit LogicRefDenied({logicRef: logicRef});
    }

    /// @notice Reverts if a consumed resource must not carry the logic reference.
    /// @param logicRef The logic reference of the consumed resource.
    function _checkLogicRefForConsumption(bytes32 logicRef) internal view {
        require(logicRefStatus(logicRef) != Status.Denied, DeniedLogicRef(logicRef));
    }

    /// @notice Reverts if a created resource must not carry the logic reference.
    /// @param logicRef The logic reference of the created resource.
    function _checkLogicRefForCreation(bytes32 logicRef) internal view {
        Status status = logicRefStatus(logicRef);
        require(status != Status.Denied, DeniedLogicRef(logicRef));
        require(status != Status.Deprecated, DeprecatedLogicRef(logicRef));
    }

    /// @notice Returns the logic reference registry storage.
    /// @return logicRefRegistryStorage The data associated with the logic reference registry.
    function _logicRefRegistryStorage()
        internal
        pure
        returns (LogicRefRegistryStorage storage logicRefRegistryStorage)
    {
        // forge-lint: disable-next-item(inline-assembly)
        assembly {
            logicRefRegistryStorage.slot := _LOGIC_REF_REGISTRY_STORAGE_SLOT
        }
    }
}
