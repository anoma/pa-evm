// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";

import {ILogicRefPolicyRegistry} from "../../src/interfaces/ILogicRefPolicyRegistry.sol";
import {LogicRefPolicyRegistry} from "../../src/state/LogicRefPolicyRegistry.sol";
import {LogicRefPolicyRegistryMock} from "../mocks/LogicRefPolicyRegistry.m.sol";

contract LogicRefPolicyRegistryTest is Test {
    LogicRefPolicyRegistryMock internal _registry;

    function setUp() public {
        _registry = LogicRefPolicyRegistryMock(
            address(
                new ERC1967Proxy(
                    address(new LogicRefPolicyRegistryMock()), abi.encodeCall(LogicRefPolicyRegistryMock.initialize, ())
                )
            )
        );
    }

    function test_all_transitions_and_resource_operations() public {
        // Row-major truth table, independent of the transition implementation.
        bool[16] memory allowed =
            [false, true, true, true, false, false, false, true, false, false, false, true, false, false, false, false];
        bool[4] memory creationAllowed = [true, false, true, false];
        bool[4] memory consumptionAllowed = [true, true, false, false];
        uint256 restricted;
        for (uint8 from = 0; from < 4; ++from) {
            for (uint8 to = 0; to < 4; ++to) {
                bytes32 ref = bytes32(uint256(from) * 4 + to + 1);
                if (from != 0) {
                    _set(ref, ILogicRefPolicyRegistry.LogicRefPolicy(from));
                    assertEq(_registry.restrictedLogicRefAtIndex(restricted++), ref);
                }
                for (uint256 op = 0; op < 2; ++op) {
                    bool consumed = op == 1;
                    if (!(consumed ? consumptionAllowed[from] : creationAllowed[from])) {
                        vm.expectRevert(
                            abi.encodeWithSelector(
                                LogicRefPolicyRegistry.ResourceWithDeniedLogicRef.selector, ref, consumed
                            )
                        );
                    }
                    _registry.checkLogicRef(ref, consumed);
                }
                if (allowed[uint256(from) * 4 + to]) {
                    vm.expectEmit(address(_registry));
                    emit ILogicRefPolicyRegistry.LogicRefPolicyChanged(
                        ref, ILogicRefPolicyRegistry.LogicRefPolicy(from), ILogicRefPolicyRegistry.LogicRefPolicy(to)
                    );
                    _set(ref, ILogicRefPolicyRegistry.LogicRefPolicy(to));
                    if (from == 0) assertEq(_registry.restrictedLogicRefAtIndex(restricted++), ref);
                    assertEq(uint8(_registry.logicRefPolicy(ref)), to);
                } else {
                    vm.expectRevert(
                        abi.encodeWithSelector(
                            LogicRefPolicyRegistry.InvalidLogicRefPolicyTransition.selector,
                            ref,
                            ILogicRefPolicyRegistry.LogicRefPolicy(from),
                            ILogicRefPolicyRegistry.LogicRefPolicy(to)
                        )
                    );
                    _set(ref, ILogicRefPolicyRegistry.LogicRefPolicy(to));
                    assertEq(uint8(_registry.logicRefPolicy(ref)), from);
                }
                assertEq(_registry.restrictedLogicRefCount(), restricted);
                assertEq(uint8(_registry.logicRefPolicy(bytes32(uint256(100)))), 0);
            }
        }
    }

    function test_batch_escalation_and_rollback() public {
        ILogicRefPolicyRegistry.PolicyUpdate[] memory updates = new ILogicRefPolicyRegistry.PolicyUpdate[](2);
        updates[0] = ILogicRefPolicyRegistry.PolicyUpdate(
            bytes32(uint256(1)), ILogicRefPolicyRegistry.LogicRefPolicy.CreationDenied
        );
        updates[1] = ILogicRefPolicyRegistry.PolicyUpdate(
            bytes32(uint256(1)), ILogicRefPolicyRegistry.LogicRefPolicy.FullyDenied
        );
        _registry.setLogicRefPolicies(updates);
        assertEq(_registry.restrictedLogicRefCount(), 1);
        assertEq(uint8(_registry.logicRefPolicy(updates[0].logicRef)), 3);

        updates[0] = ILogicRefPolicyRegistry.PolicyUpdate(
            bytes32(uint256(2)), ILogicRefPolicyRegistry.LogicRefPolicy.ConsumptionDenied
        );
        updates[1] = ILogicRefPolicyRegistry.PolicyUpdate(
            bytes32(uint256(2)), ILogicRefPolicyRegistry.LogicRefPolicy.CreationDenied
        );
        vm.expectRevert(
            abi.encodeWithSelector(
                LogicRefPolicyRegistry.InvalidLogicRefPolicyTransition.selector,
                updates[0].logicRef,
                ILogicRefPolicyRegistry.LogicRefPolicy.ConsumptionDenied,
                ILogicRefPolicyRegistry.LogicRefPolicy.CreationDenied
            )
        );
        _registry.setLogicRefPolicies(updates);
        assertEq(_registry.restrictedLogicRefCount(), 1);
        assertEq(uint8(_registry.logicRefPolicy(updates[0].logicRef)), 0);
    }

    function test_invalid_inputs_and_empty_batch() public {
        _registry.setLogicRefPolicies(new ILogicRefPolicyRegistry.PolicyUpdate[](0));
        vm.expectRevert(LogicRefPolicyRegistry.ZeroLogicRefNotAllowed.selector);
        _set(bytes32(0), ILogicRefPolicyRegistry.LogicRefPolicy.FullyDenied);
        // Encode uint8 directly to bypass Solidity's enum conversion check in this test.
        bytes memory data = abi.encodeWithSelector(
            ILogicRefPolicyRegistry.setLogicRefPolicies.selector,
            uint256(32),
            uint256(1),
            bytes32(uint256(1)),
            uint256(4)
        );
        // solhint-disable-next-line avoid-low-level-calls
        (bool success,) = address(_registry).call(data);
        assertFalse(success);
        assertEq(_registry.restrictedLogicRefCount(), 0);
    }

    function _set(bytes32 ref, ILogicRefPolicyRegistry.LogicRefPolicy policy) internal {
        ILogicRefPolicyRegistry.PolicyUpdate[] memory updates = new ILogicRefPolicyRegistry.PolicyUpdate[](1);
        updates[0] = ILogicRefPolicyRegistry.PolicyUpdate(ref, policy);
        _registry.setLogicRefPolicies(updates);
    }
}
