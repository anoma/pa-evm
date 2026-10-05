// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";

import {ILogicRefStatuses} from "../interfaces/ILogicRefStatuses.sol";

/// @title LogicRefStatuses
/// @author Anoma Foundation, 2026
/// @notice The logic reference statuses being inherited by the protocol adapter.
/// @dev A status only becomes stricter, from active to deprecated to denied. No function removes a listed logic
/// reference.
/// @custom:security-contact security@anoma.foundation
abstract contract LogicRefStatuses is ILogicRefStatuses, Initializable {
    /// @custom:storage-location erc7201:anoma.storage.LogicRefStatuses
    struct LogicRefStatusesStorage {
        bytes32[] _listedLogicRefs;
        mapping(bytes32 logicRef => Status status) _statuses;
    }

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.LogicRefStatuses")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _LOGIC_REF_STATUSES_STORAGE_SLOT =
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

    /// @inheritdoc ILogicRefStatuses
    function getLogicRefStatus(bytes32 logicRef) external view override returns (Status status) {
        LogicRefStatusesStorage storage $ = _getLogicRefStatusesStorage();

        status = $._statuses[logicRef];
    }

    /// @inheritdoc ILogicRefStatuses
    function listedLogicRefCount() external view override returns (uint256 count) {
        LogicRefStatusesStorage storage $ = _getLogicRefStatusesStorage();

        count = $._listedLogicRefs.length;
    }

    /// @inheritdoc ILogicRefStatuses
    function listedLogicRefAtIndex(uint256 index) external view override returns (bytes32 logicRef) {
        LogicRefStatusesStorage storage $ = _getLogicRefStatusesStorage();

        logicRef = $._listedLogicRefs[index];
    }

    /// @notice Initializes the LogicRefStatuses contract.
    /// @dev Every logic reference starts active, so the contract requires no setup. The function exists for
    /// consistency with the OpenZeppelin initializer convention.
    // forge-lint: disable-next-line(mixed-case-function)
    function __LogicRefStatuses_init() internal onlyInitializing {}

    /// @notice Deprecates an active logic reference and emits the `LogicRefDeprecated` event.
    /// @param logicRef The logic reference to deprecate.
    function _deprecateLogicRef(bytes32 logicRef) internal {
        require(logicRef != bytes32(0), ZeroLogicRefNotAllowed());

        LogicRefStatusesStorage storage $ = _getLogicRefStatusesStorage();

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

        LogicRefStatusesStorage storage $ = _getLogicRefStatusesStorage();

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
        LogicRefStatusesStorage storage $ = _getLogicRefStatusesStorage();

        require($._statuses[logicRef] != Status.Denied, DeniedLogicRef(logicRef));
    }

    /// @notice Reverts if a created resource must not carry the logic reference.
    /// @param logicRef The logic reference of the created resource.
    function _checkCreatedLogicRef(bytes32 logicRef) internal view {
        LogicRefStatusesStorage storage $ = _getLogicRefStatusesStorage();

        Status status = $._statuses[logicRef];
        require(status != Status.Denied, DeniedLogicRef(logicRef));
        require(status != Status.Deprecated, DeprecatedLogicRef(logicRef));
    }

    /// @notice Returns the storage from the logic reference statuses storage location.
    /// @return logicRefStatusesStorage The data associated with the logic reference statuses storage.
    function _getLogicRefStatusesStorage()
        internal
        pure
        returns (LogicRefStatusesStorage storage logicRefStatusesStorage)
    {
        // forge-lint: disable-next-item(inline-assembly)
        assembly {
            logicRefStatusesStorage.slot := _LOGIC_REF_STATUSES_STORAGE_SLOT
        }
    }
}
