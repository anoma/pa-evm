// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {UUPSUpgradeable} from "@openzeppelin-contracts-5.7.0/proxy/utils/UUPSUpgradeable.sol";

import {LogicRefStatusesMock} from "./LogicRefStatuses.m.sol";

contract LogicRefStatusesUpgradeMock is LogicRefStatusesMock, UUPSUpgradeable {
    function checkConsumedLogicRef(bytes32 logicRef) external view {
        _checkConsumedLogicRef(logicRef);
    }

    function checkCreatedLogicRef(bytes32 logicRef) external view {
        _checkCreatedLogicRef(logicRef);
    }

    function _authorizeUpgrade(address) internal override {} // solhint-disable-line no-empty-blocks
}
