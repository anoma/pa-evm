// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {LogicRefStatuses} from "../../src/state/LogicRefStatuses.sol";

contract LogicRefStatusesMock is LogicRefStatuses {
    function initialize() external initializer {
        __LogicRefStatuses_init();
    }

    function deprecateLogicRef(bytes32 logicRef) external {
        _deprecateLogicRef(logicRef);
    }

    function denyLogicRef(bytes32 logicRef) external {
        _denyLogicRef(logicRef);
    }
}
