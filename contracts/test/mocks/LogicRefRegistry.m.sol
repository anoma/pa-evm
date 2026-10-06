// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {LogicRefRegistry} from "../../src/state/LogicRefRegistry.sol";

contract LogicRefRegistryMock is LogicRefRegistry {
    function initialize() external initializer {
        __LogicRefRegistry_init();
    }

    // The mock lets anyone change statuses; `ProtocolAdapter` allows only the owner.
    // solhint-disable-next-line no-empty-blocks
    function _authorizeLogicRefRegistryChange() internal override {}
}
