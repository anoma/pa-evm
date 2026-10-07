// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";

import {ILogicRefDenylist} from "../interfaces/ILogicRefDenylist.sol";

/// @title LogicRefDenylist
/// @author Anoma Foundation, 2026
/// @notice The denylists of logic references being inherited by the protocol adapter: one for consumed resources and
/// one for created resources.
/// @dev No function removes an entry.
/// @custom:security-contact security@anoma.foundation
abstract contract LogicRefDenylist is ILogicRefDenylist, Initializable {
    /// @notice Whether each denylist contains a logic reference, in one storage slot so that a check reads one slot.
    struct Denial {
        bool consumed; //  ┐   1
        bool created; //   ┘ + 1 = 2
    }

    /// @custom:storage-location erc7201:anoma.storage.LogicRefDenylist
    struct LogicRefDenylistStorage {
        mapping(bytes32 logicRef => Denial denial) _denials;
    }

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.LogicRefDenylist")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _LOGIC_REF_DENYLIST_STORAGE_SLOT =
        0x4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900;

    error ZeroLogicRefNotAllowed();
    error LogicRefAlreadyDenied(bytes32 logicRef, bool consumed);
    error ResourceWithDeniedLogicRef(bytes32 logicRef, bool consumed);

    /// @notice The constructor disabling the initializers on the implementation contract.
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @inheritdoc ILogicRefDenylist
    function denyLogicRefs(DeniedLogicRef[] calldata logicRefs) external override {
        _authorizeLogicRefDenylistChange();

        uint256 count = logicRefs.length;
        for (uint256 i = 0; i < count; ++i) {
            _denyLogicRef({logicRef: logicRefs[i].logicRef, consumed: logicRefs[i].consumed});
        }
    }

    /// @inheritdoc ILogicRefDenylist
    function isLogicRefDenied(bytes32 logicRef, bool consumed) external view override returns (bool isDenied) {
        isDenied = _isLogicRefDenied(logicRef, consumed);
    }

    /// @notice Initializes the LogicRefDenylist contract.
    /// @dev The denylists start empty and require no setup. The function exists for consistency with the OpenZeppelin
    /// initializer convention.
    // forge-lint: disable-next-line(mixed-case-function)
    function __LogicRefDenylist_init() internal onlyInitializing {}

    /// @notice Reverts unless the caller is allowed to add logic references to the denylists.
    function _authorizeLogicRefDenylistChange() internal virtual;

    /// @notice Adds a logic reference to a denylist and emits the `LogicRefDenied` event.
    /// @param logicRef The logic reference to deny.
    /// @param consumed `true` for the denylist for consumed resources, `false` for the one for created resources.
    function _denyLogicRef(bytes32 logicRef, bool consumed) internal {
        require(logicRef != bytes32(0), ZeroLogicRefNotAllowed());

        require(!_isLogicRefDenied(logicRef, consumed), LogicRefAlreadyDenied(logicRef, consumed));

        Denial storage denial = _getLogicRefDenylistStorage()._denials[logicRef];
        if (consumed) {
            denial.consumed = true;
        } else {
            denial.created = true;
        }

        emit LogicRefDenied({logicRef: logicRef, consumed: consumed});
    }

    /// @notice Checks that the denylist for the side of a resource does not contain its logic reference.
    /// @param logicRef The logic reference of the resource.
    /// @param consumed `true` for a consumed resource, `false` for a created resource.
    function _checkLogicRefNotDenied(bytes32 logicRef, bool consumed) internal view {
        require(
            !_isLogicRefDenied(logicRef, consumed), ResourceWithDeniedLogicRef({logicRef: logicRef, consumed: consumed})
        );
    }

    /// @notice Returns whether the denylist for consumed or for created resources contains a logic reference.
    /// @param logicRef The logic reference to check.
    /// @param consumed `true` for the denylist for consumed resources, `false` for the one for created resources.
    /// @return isDenied Whether the denylist contains the logic reference or not.
    function _isLogicRefDenied(bytes32 logicRef, bool consumed) internal view returns (bool isDenied) {
        Denial storage denial = _getLogicRefDenylistStorage()._denials[logicRef];

        isDenied = consumed ? denial.consumed : denial.created;
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
