// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {LogicRefRegistry} from "../../src/state/LogicRefRegistry.sol";

contract LogicRefRegistryMock is LogicRefRegistry {
    function initialize() external initializer {
        __LogicRefRegistry_init();
    }

    function deprecateLogicRef(bytes32 logicRef) external {
        _deprecateLogicRef(logicRef);
    }

    function denyLogicRef(bytes32 logicRef) external {
        _denyLogicRef(logicRef);
    }
}
