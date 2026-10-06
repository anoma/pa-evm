// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {ILogicRefRegistry} from "../../src/interfaces/ILogicRefRegistry.sol";
import {LogicRefRegistry} from "../../src/state/LogicRefRegistry.sol";
import {LogicRefRegistryMock} from "../mocks/LogicRefRegistry.m.sol";

contract LogicRefRegistryTest is Test {
    bytes32 internal constant _EXAMPLE_LOGIC_REF = bytes32(uint256(1));

    LogicRefRegistryMock internal _logicRefRegistry;

    function setUp() public {
        _logicRefRegistry = new LogicRefRegistryMock();
    }

    function testFuzz_deprecateLogicRef_deprecates_the_logic_ref(bytes32 logicRef) public {
        vm.assume(logicRef != bytes32(0));

        _logicRefRegistry.deprecateLogicRef(logicRef);

        _assertStatus(logicRef, ILogicRefRegistry.Status.Deprecated, "the logic ref should be deprecated");
    }

    function test_deprecateLogicRef_emits_the_LogicRefDeprecated_event() public {
        vm.expectEmit(address(_logicRefRegistry));
        emit ILogicRefRegistry.LogicRefDeprecated({logicRef: _EXAMPLE_LOGIC_REF});
        _logicRefRegistry.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_a_deprecated_logic_ref() public {
        _logicRefRegistry.deprecateLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefRegistry.LogicRefAlreadyDeprecated.selector, _EXAMPLE_LOGIC_REF),
            address(_logicRefRegistry)
        );
        _logicRefRegistry.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_a_denied_logic_ref() public {
        _logicRefRegistry.denyLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefRegistry.LogicRefAlreadyDenied.selector, _EXAMPLE_LOGIC_REF),
            address(_logicRefRegistry)
        );
        _logicRefRegistry.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_the_zero_logic_ref() public {
        vm.expectRevert(LogicRefRegistry.ZeroLogicRefNotAllowed.selector, address(_logicRefRegistry));
        _logicRefRegistry.deprecateLogicRef(bytes32(0));
    }

    function testFuzz_denyLogicRef_denies_the_logic_ref(bytes32 logicRef) public {
        vm.assume(logicRef != bytes32(0));

        _logicRefRegistry.denyLogicRef(logicRef);

        _assertStatus(logicRef, ILogicRefRegistry.Status.Denied, "the logic ref should be denied");
    }

    function test_denyLogicRef_denies_a_deprecated_logic_ref() public {
        _logicRefRegistry.deprecateLogicRef(_EXAMPLE_LOGIC_REF);

        _logicRefRegistry.denyLogicRef(_EXAMPLE_LOGIC_REF);

        _assertStatus(_EXAMPLE_LOGIC_REF, ILogicRefRegistry.Status.Denied, "the logic ref should be denied");
        assertEq(_logicRefRegistry.nonActiveLogicRefCount(), 1, "the logic ref should be listed once");
    }

    function test_denyLogicRef_emits_the_LogicRefDenied_event() public {
        vm.expectEmit(address(_logicRefRegistry));
        emit ILogicRefRegistry.LogicRefDenied({logicRef: _EXAMPLE_LOGIC_REF});
        _logicRefRegistry.denyLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_denyLogicRef_reverts_on_a_denied_logic_ref() public {
        _logicRefRegistry.denyLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefRegistry.LogicRefAlreadyDenied.selector, _EXAMPLE_LOGIC_REF),
            address(_logicRefRegistry)
        );
        _logicRefRegistry.denyLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_denyLogicRef_reverts_on_the_zero_logic_ref() public {
        vm.expectRevert(LogicRefRegistry.ZeroLogicRefNotAllowed.selector, address(_logicRefRegistry));
        _logicRefRegistry.denyLogicRef(bytes32(0));
    }

    function test_nonActiveLogicRefCount_counts_deprecated_and_denied_logic_refs() public {
        assertEq(_logicRefRegistry.nonActiveLogicRefCount(), 0, "no logic ref should be non-active");

        uint256 n = 10;
        for (uint256 i = 1; i < n; ++i) {
            if (i % 2 == 0) {
                _logicRefRegistry.deprecateLogicRef(bytes32(i));
            } else {
                _logicRefRegistry.denyLogicRef(bytes32(i));
            }
            assertEq(_logicRefRegistry.nonActiveLogicRefCount(), i, "the count should match non-active logic refs");
        }
    }

    function test_nonActiveLogicRefAtIndex_preserves_insertion_order() public {
        uint256 n = 10;
        for (uint256 i = 0; i < n; ++i) {
            if (i % 2 == 0) {
                _logicRefRegistry.deprecateLogicRef(bytes32(n - i));
            } else {
                _logicRefRegistry.denyLogicRef(bytes32(n - i));
            }
        }

        for (uint256 i = 0; i < n; ++i) {
            assertEq(
                _logicRefRegistry.nonActiveLogicRefAtIndex(i), bytes32(n - i), "the non-active logic ref should match"
            );
        }
    }

    function testFuzz_logicRefStatus_defaults_to_active(bytes32 logicRef) public view {
        _assertStatus(logicRef, ILogicRefRegistry.Status.Active, "an unlisted logic ref should be active");
    }

    function _assertStatus(bytes32 logicRef, ILogicRefRegistry.Status expected, string memory message) internal view {
        assertEq(uint8(_logicRefRegistry.logicRefStatus(logicRef)), uint8(expected), message);
    }
}
