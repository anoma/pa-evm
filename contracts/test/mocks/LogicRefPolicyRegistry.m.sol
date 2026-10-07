// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {LogicRefPolicyRegistry} from "../../src/state/LogicRefPolicyRegistry.sol";

contract LogicRefPolicyRegistryMock is LogicRefPolicyRegistry {
    function initialize() external initializer {
        __LogicRefPolicyRegistry_init();
    }

    function checkLogicRef(bytes32 logicRef, bool consumed) external view {
        _requireLogicRefPoliciesInitialized();
        _checkLogicRefNotDenied(logicRef, consumed);
    }

    // The adapter's authorization hook restricts updates to its owner.
    // solhint-disable-next-line no-empty-blocks
    function _authorizeLogicRefPolicyChange() internal override {}
}
