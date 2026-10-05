// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";
import {EnumerableSet} from "@openzeppelin-contracts-5.7.0/utils/structs/EnumerableSet.sol";

/// @dev Reproduces the denylist storage at 4986aa1f635722b8b0f08aefe841752551fe2053.
contract LegacyLogicRefDenylistMock is Initializable, UUPSUpgradeable {
    using EnumerableSet for EnumerableSet.Bytes32Set;

    /// @custom:storage-location erc7201:anoma.storage.LogicRefDenylist
    struct LogicRefDenylistStorage {
        EnumerableSet.Bytes32Set _deniedLogicRefs;
    }

    error ZeroLogicRefNotAllowed();
    error LogicRefAlreadyDenied(bytes32 logicRef);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize() external initializer {} // solhint-disable-line no-empty-blocks

    function denyLogicRef(bytes32 logicRef) external {
        require(logicRef != bytes32(0), ZeroLogicRefNotAllowed());
        require(_getStorage()._deniedLogicRefs.add(logicRef), LogicRefAlreadyDenied(logicRef));
    }

    function isLogicRefDenied(bytes32 logicRef) external view returns (bool isDenied) {
        isDenied = _getStorage()._deniedLogicRefs.contains(logicRef);
    }

    function _authorizeUpgrade(address) internal override {} // solhint-disable-line no-empty-blocks

    function _getStorage() internal pure returns (LogicRefDenylistStorage storage $) {
        // solhint-disable-next-line no-inline-assembly
        assembly {
            $.slot := 0x4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900
        }
    }
}
