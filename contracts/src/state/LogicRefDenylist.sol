// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";

import {EnumerableSet} from "@openzeppelin-contracts-5.7.0/utils/structs/EnumerableSet.sol";

import {ILogicRefDenylist} from "../interfaces/ILogicRefDenylist.sol";

/// @title LogicRefDenylist
/// @author Anoma Foundation, 2026
/// @notice A denylist of logic references being inherited by the protocol adapter.
/// @dev The implementation is based on OpenZeppelin's `EnumerableSet` implementation. No function removes an entry.
/// @custom:security-contact security@anoma.foundation
abstract contract LogicRefDenylist is ILogicRefDenylist, Initializable {
    using EnumerableSet for EnumerableSet.Bytes32Set;

    /// @custom:storage-location erc7201:anoma.storage.LogicRefDenylist
    struct LogicRefDenylistStorage {
        EnumerableSet.Bytes32Set _deniedLogicRefs;
    }

    // keccak256(abi.encode(uint256(keccak256("anoma.storage.LogicRefDenylist")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 internal constant _LOGIC_REF_DENYLIST_STORAGE_SLOT =
        0x4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900;

    error ZeroLogicRefNotAllowed();
    error LogicRefAlreadyDenied(bytes32 logicRef);
    error DeniedLogicRef(bytes32 logicRef);

    /// @notice The constructor disabling the initializers on the implementation contract.
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /// @inheritdoc ILogicRefDenylist
    function isLogicRefDenied(bytes32 logicRef) external view override returns (bool isDenied) {
        isDenied = _isLogicRefDenied(logicRef);
    }

    /// @inheritdoc ILogicRefDenylist
    function deniedLogicRefCount() external view override returns (uint256 count) {
        LogicRefDenylistStorage storage $ = _getLogicRefDenylistStorage();

        count = $._deniedLogicRefs.length();
    }

    /// @inheritdoc ILogicRefDenylist
    function deniedLogicRefAtIndex(uint256 index) external view override returns (bytes32 logicRef) {
        LogicRefDenylistStorage storage $ = _getLogicRefDenylistStorage();

        logicRef = $._deniedLogicRefs.at(index);
    }

    /// @notice Initializes the LogicRefDenylist contract.
    /// @dev The denylist starts empty and requires no setup. The function exists for consistency with the OpenZeppelin
    /// initializer convention.
    // solhint-disable-next-line func-name-mixedcase, no-empty-blocks
    function __LogicRefDenylist_init() internal onlyInitializing {}

    /// @notice Adds a logic reference to the denylist and emits the `LogicRefDenied` event.
    /// @param logicRef The logic reference to deny.
    function _denyLogicRef(bytes32 logicRef) internal {
        require(logicRef != bytes32(0), ZeroLogicRefNotAllowed());

        LogicRefDenylistStorage storage $ = _getLogicRefDenylistStorage();

        require($._deniedLogicRefs.add(logicRef), LogicRefAlreadyDenied(logicRef));

        emit LogicRefDenied({logicRef: logicRef});
    }

    /// @notice Checks whether the denylist contains a logic reference.
    /// @param logicRef The logic reference to check.
    /// @return isDenied Whether the logic reference is denied or not.
    function _isLogicRefDenied(bytes32 logicRef) internal view returns (bool isDenied) {
        LogicRefDenylistStorage storage $ = _getLogicRefDenylistStorage();

        isDenied = $._deniedLogicRefs.contains(logicRef);
    }

    /// @notice Returns the storage from the logic reference denylist storage location.
    /// @return logicRefDenylistStorage The data associated with the logic reference denylist storage.
    function _getLogicRefDenylistStorage()
        internal
        pure
        returns (LogicRefDenylistStorage storage logicRefDenylistStorage)
    {
        /* solhint-disable no-inline-assembly */

        // slither-disable-next-line assembly
        assembly {
            logicRefDenylistStorage.slot := _LOGIC_REF_DENYLIST_STORAGE_SLOT
        }

        /* solhint-enable no-inline-assembly */
    }
}
