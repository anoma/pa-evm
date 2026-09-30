// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {LogicRefDenylist} from "../../src/state/LogicRefDenylist.sol";

contract LogicRefDenylistMock is LogicRefDenylist {
    function initialize() external initializer {
        __LogicRefDenylist_init();
    }

    function denyLogicRef(bytes32 logicRef) external {
        _denyLogicRef(logicRef);
    }
}
