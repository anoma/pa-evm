// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {ILogicRefStatuses} from "../../src/interfaces/ILogicRefStatuses.sol";
import {LogicRefStatuses} from "../../src/state/LogicRefStatuses.sol";
import {LogicRefStatusesMock} from "../mocks/LogicRefStatuses.m.sol";

contract LogicRefStatusesTest is Test {
    bytes32 internal constant _EXAMPLE_LOGIC_REF = bytes32(uint256(1));

    LogicRefStatusesMock internal _logicRefStatuses;

    function setUp() public {
        _logicRefStatuses = new LogicRefStatusesMock();
    }

    function testFuzz_deprecateLogicRef_deprecates_the_logic_ref(bytes32 logicRef) public {
        vm.assume(logicRef != bytes32(0));

        _logicRefStatuses.deprecateLogicRef(logicRef);

        _assertStatus(logicRef, ILogicRefStatuses.Status.Deprecated, "the logic ref should be deprecated");
    }

    function test_deprecateLogicRef_emits_the_LogicRefDeprecated_event() public {
        vm.expectEmit(address(_logicRefStatuses));
        emit ILogicRefStatuses.LogicRefDeprecated({logicRef: _EXAMPLE_LOGIC_REF});
        _logicRefStatuses.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_a_deprecated_logic_ref() public {
        _logicRefStatuses.deprecateLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefStatuses.LogicRefAlreadyDeprecated.selector, _EXAMPLE_LOGIC_REF),
            address(_logicRefStatuses)
        );
        _logicRefStatuses.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_a_denied_logic_ref() public {
        _logicRefStatuses.denyLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefStatuses.LogicRefAlreadyDenied.selector, _EXAMPLE_LOGIC_REF),
            address(_logicRefStatuses)
        );
        _logicRefStatuses.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_the_zero_logic_ref() public {
        vm.expectRevert(LogicRefStatuses.ZeroLogicRefNotAllowed.selector, address(_logicRefStatuses));
        _logicRefStatuses.deprecateLogicRef(bytes32(0));
    }

    function testFuzz_denyLogicRef_denies_the_logic_ref(bytes32 logicRef) public {
        vm.assume(logicRef != bytes32(0));

        _logicRefStatuses.denyLogicRef(logicRef);

        _assertStatus(logicRef, ILogicRefStatuses.Status.Denied, "the logic ref should be denied");
    }

    function test_denyLogicRef_denies_a_deprecated_logic_ref() public {
        _logicRefStatuses.deprecateLogicRef(_EXAMPLE_LOGIC_REF);

        _logicRefStatuses.denyLogicRef(_EXAMPLE_LOGIC_REF);

        _assertStatus(_EXAMPLE_LOGIC_REF, ILogicRefStatuses.Status.Denied, "the logic ref should be denied");
        assertEq(_logicRefStatuses.listedLogicRefCount(), 1, "the logic ref should be listed once");
    }

    function test_denyLogicRef_emits_the_LogicRefDenied_event() public {
        vm.expectEmit(address(_logicRefStatuses));
        emit ILogicRefStatuses.LogicRefDenied({logicRef: _EXAMPLE_LOGIC_REF});
        _logicRefStatuses.denyLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_denyLogicRef_reverts_on_a_denied_logic_ref() public {
        _logicRefStatuses.denyLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefStatuses.LogicRefAlreadyDenied.selector, _EXAMPLE_LOGIC_REF),
            address(_logicRefStatuses)
        );
        _logicRefStatuses.denyLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_denyLogicRef_reverts_on_the_zero_logic_ref() public {
        vm.expectRevert(LogicRefStatuses.ZeroLogicRefNotAllowed.selector, address(_logicRefStatuses));
        _logicRefStatuses.denyLogicRef(bytes32(0));
    }

    function test_listedLogicRefCount_counts_deprecated_and_denied_logic_refs() public {
        assertEq(_logicRefStatuses.listedLogicRefCount(), 0, "no logic ref should be listed");

        uint256 n = 10;
        for (uint256 i = 1; i < n; ++i) {
            if (i % 2 == 0) {
                _logicRefStatuses.deprecateLogicRef(bytes32(i));
            } else {
                _logicRefStatuses.denyLogicRef(bytes32(i));
            }
            assertEq(
                _logicRefStatuses.listedLogicRefCount(), i, "the count should match the number of listed logic refs"
            );
        }
    }

    function test_listedLogicRefAtIndex_returns_the_logic_refs_in_the_order_they_were_listed() public {
        uint256 n = 10;
        for (uint256 i = 0; i < n; ++i) {
            if (i % 2 == 0) {
                _logicRefStatuses.deprecateLogicRef(bytes32(n - i));
            } else {
                _logicRefStatuses.denyLogicRef(bytes32(n - i));
            }
        }

        for (uint256 i = 0; i < n; ++i) {
            assertEq(
                _logicRefStatuses.listedLogicRefAtIndex(i), bytes32(n - i), "the logic ref at the index should match"
            );
        }
    }

    function testFuzz_getLogicRefStatus_returns_active_for_an_unlisted_logic_ref(bytes32 logicRef) public view {
        _assertStatus(logicRef, ILogicRefStatuses.Status.Active, "an unlisted logic ref should be active");
    }

    function _assertStatus(bytes32 logicRef, ILogicRefStatuses.Status expected, string memory message) internal view {
        assertEq(uint8(_logicRefStatuses.getLogicRefStatus(logicRef)), uint8(expected), message);
    }
}
