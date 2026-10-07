// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";
import {EnumerableSet} from "@openzeppelin-contracts-5.7.0/utils/structs/EnumerableSet.sol";
import {OwnableUpgradeable} from "@openzeppelin-contracts-upgradeable-5.7.0/access/OwnableUpgradeable.sol";

/// @dev Minimal upgrade fixture retaining the two-set layout and initializer version from next at 7deb37d.
contract LegacyLogicRefDenylistMock is OwnableUpgradeable, UUPSUpgradeable {
    using EnumerableSet for EnumerableSet.Bytes32Set;

    /// @custom:storage-location erc7201:anoma.storage.LogicRefDenylist
    struct LogicRefDenylistStorage {
        EnumerableSet.Bytes32Set _deniedConsumedLogicRefs;
        EnumerableSet.Bytes32Set _deniedCreatedLogicRefs;
    }

    constructor() {
        _disableInitializers();
    }

    function initialize(address owner) external initializer {
        __Ownable_init(owner);
    }

    function deny(bytes32 ref, bool consumed) external onlyOwner {
        LogicRefDenylistStorage storage legacy;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            legacy.slot := 0x4236e6c1c068f5e3b8927e001e7112af467e55c6094df039f1d58b8530a62900
        }
        if (consumed) legacy._deniedConsumedLogicRefs.add(ref);
        else legacy._deniedCreatedLogicRefs.add(ref);
    }

    // solhint-disable-next-line no-empty-blocks
    function _authorizeUpgrade(address) internal override onlyOwner {}
}
