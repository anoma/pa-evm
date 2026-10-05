// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";

import {ILogicRefDenylist} from "../interfaces/ILogicRefDenylist.sol";

/// @title LogicRefDenylist
/// @author Anoma Foundation, 2026
/// @notice A denylist of logic references being inherited by the protocol adapter.
/// @dev A status only becomes stricter, from active to deprecated to denied. No function removes an entry.
/// @custom:security-contact security@anoma.foundation
abstract contract LogicRefDenylist is ILogicRefDenylist, Initializable {
    /// @custom:storage-location erc7201:anoma.storage.LogicRefDenylist
    struct LogicRefDenylistStorage {
        bytes32[] _listedLogicRefs;
        mapping(bytes32 logicRef => Status status) _statuses;
    }

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.LogicRefDenylist")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _LOGIC_REF_DENYLIST_STORAGE_SLOT =
        0x4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900;

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

    /// @inheritdoc ILogicRefDenylist
    function getLogicRefStatus(bytes32 logicRef) external view override returns (Status status) {
        LogicRefDenylistStorage storage $ = _getLogicRefDenylistStorage();

        status = $._statuses[logicRef];
    }

    /// @inheritdoc ILogicRefDenylist
    function listedLogicRefCount() external view override returns (uint256 count) {
        LogicRefDenylistStorage storage $ = _getLogicRefDenylistStorage();

        count = $._listedLogicRefs.length;
    }

    /// @inheritdoc ILogicRefDenylist
    function listedLogicRefAtIndex(uint256 index) external view override returns (bytes32 logicRef) {
        LogicRefDenylistStorage storage $ = _getLogicRefDenylistStorage();

        logicRef = $._listedLogicRefs[index];
    }

    /// @notice Initializes the LogicRefDenylist contract.
    /// @dev The denylist starts empty and requires no setup. The function exists for consistency with the OpenZeppelin
    /// initializer convention.
    // forge-lint: disable-next-line(mixed-case-function)
    function __LogicRefDenylist_init() internal onlyInitializing {}

    /// @notice Deprecates an active logic reference and emits the `LogicRefDeprecated` event.
    /// @param logicRef The logic reference to deprecate.
    function _deprecateLogicRef(bytes32 logicRef) internal {
        require(logicRef != bytes32(0), ZeroLogicRefNotAllowed());

        LogicRefDenylistStorage storage $ = _getLogicRefDenylistStorage();

        Status status = $._statuses[logicRef];
        require(status != Status.Deprecated, LogicRefAlreadyDeprecated(logicRef));
        require(status != Status.Denied, LogicRefAlreadyDenied(logicRef));

        $._listedLogicRefs.push(logicRef);
        $._statuses[logicRef] = Status.Deprecated;

        emit LogicRefDeprecated({logicRef: logicRef});
    }

    /// @notice Denies an active or deprecated logic reference and emits the `LogicRefDenied` event.
    /// @param logicRef The logic reference to deny.
    function _denyLogicRef(bytes32 logicRef) internal {
        require(logicRef != bytes32(0), ZeroLogicRefNotAllowed());

        LogicRefDenylistStorage storage $ = _getLogicRefDenylistStorage();

        Status status = $._statuses[logicRef];
        require(status != Status.Denied, LogicRefAlreadyDenied(logicRef));

        if (status == Status.Active) {
            $._listedLogicRefs.push(logicRef);
        }
        $._statuses[logicRef] = Status.Denied;

        emit LogicRefDenied({logicRef: logicRef});
    }

    /// @notice Reverts if a consumed resource must not carry the logic reference.
    /// @param logicRef The logic reference of the consumed resource.
    function _checkConsumedLogicRef(bytes32 logicRef) internal view {
        LogicRefDenylistStorage storage $ = _getLogicRefDenylistStorage();

        require($._statuses[logicRef] != Status.Denied, DeniedLogicRef(logicRef));
    }

    /// @notice Reverts if a created resource must not carry the logic reference.
    /// @param logicRef The logic reference of the created resource.
    function _checkCreatedLogicRef(bytes32 logicRef) internal view {
        LogicRefDenylistStorage storage $ = _getLogicRefDenylistStorage();

        Status status = $._statuses[logicRef];
        require(status != Status.Denied, DeniedLogicRef(logicRef));
        require(status != Status.Deprecated, DeprecatedLogicRef(logicRef));
    }

    /// @notice Returns the storage from the logic reference denylist storage location.
    /// @return logicRefDenylistStorage The data associated with the logic reference denylist storage.
    function _getLogicRefDenylistStorage()
        internal
        pure
        returns (LogicRefDenylistStorage storage logicRefDenylistStorage)
    {
        // forge-lint: disable-next-item(inline-assembly)
        assembly {
            logicRefDenylistStorage.slot := _LOGIC_REF_DENYLIST_STORAGE_SLOT
        }
    }
}
