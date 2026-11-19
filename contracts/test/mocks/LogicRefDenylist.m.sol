// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {LogicRefDenylist} from "../../src/state/LogicRefDenylist.sol";

contract LogicRefDenylistMock is LogicRefDenylist {
    function initialize() external initializer {
        __LogicRefDenylist_init();
    }

    function denyLogicRef(bytes32 logicRef, bool consumed) external {
        _denyLogicRef({logicRef: logicRef, consumed: consumed});
    }

    // The mock lets anyone deny a logic reference; `ProtocolAdapter` allows only the owner.
    // solhint-disable-next-line no-empty-blocks
    function _authorizeLogicRefDenylistChange() internal override {}
}
