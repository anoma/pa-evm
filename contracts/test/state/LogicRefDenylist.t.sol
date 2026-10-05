// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {ILogicRefDenylist} from "../../src/interfaces/ILogicRefDenylist.sol";
import {LogicRefDenylist} from "../../src/state/LogicRefDenylist.sol";
import {LogicRefDenylistMock} from "../mocks/LogicRefDenylist.m.sol";

contract LogicRefDenylistTest is Test {
    bytes32 internal constant _EXAMPLE_LOGIC_REF = bytes32(uint256(1));

    LogicRefDenylistMock internal _denylist;

    function setUp() public {
        _denylist = new LogicRefDenylistMock();
    }

    function testFuzz_deprecateLogicRef_deprecates_the_logic_ref(bytes32 logicRef) public {
        vm.assume(logicRef != bytes32(0));

        _denylist.deprecateLogicRef(logicRef);

        _assertStatus(logicRef, ILogicRefDenylist.Status.Deprecated, "the logic ref should be deprecated");
    }

    function test_deprecateLogicRef_emits_the_LogicRefDeprecated_event() public {
        vm.expectEmit(address(_denylist));
        emit ILogicRefDenylist.LogicRefDeprecated({logicRef: _EXAMPLE_LOGIC_REF});
        _denylist.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_a_deprecated_logic_ref() public {
        _denylist.deprecateLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefDenylist.LogicRefAlreadyDeprecated.selector, _EXAMPLE_LOGIC_REF),
            address(_denylist)
        );
        _denylist.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_a_denied_logic_ref() public {
        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefDenylist.LogicRefAlreadyDenied.selector, _EXAMPLE_LOGIC_REF),
            address(_denylist)
        );
        _denylist.deprecateLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_deprecateLogicRef_reverts_on_the_zero_logic_ref() public {
        vm.expectRevert(LogicRefDenylist.ZeroLogicRefNotAllowed.selector, address(_denylist));
        _denylist.deprecateLogicRef(bytes32(0));
    }

    function testFuzz_denyLogicRef_denies_the_logic_ref(bytes32 logicRef) public {
        vm.assume(logicRef != bytes32(0));

        _denylist.denyLogicRef(logicRef);

        _assertStatus(logicRef, ILogicRefDenylist.Status.Denied, "the logic ref should be denied");
    }

    function test_denyLogicRef_denies_a_deprecated_logic_ref() public {
        _denylist.deprecateLogicRef(_EXAMPLE_LOGIC_REF);

        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF);

        _assertStatus(_EXAMPLE_LOGIC_REF, ILogicRefDenylist.Status.Denied, "the logic ref should be denied");
        assertEq(_denylist.listedLogicRefCount(), 1, "the logic ref should be listed once");
    }

    function test_denyLogicRef_emits_the_LogicRefDenied_event() public {
        vm.expectEmit(address(_denylist));
        emit ILogicRefDenylist.LogicRefDenied({logicRef: _EXAMPLE_LOGIC_REF});
        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_denyLogicRef_reverts_on_a_denied_logic_ref() public {
        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefDenylist.LogicRefAlreadyDenied.selector, _EXAMPLE_LOGIC_REF),
            address(_denylist)
        );
        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_denyLogicRef_reverts_on_the_zero_logic_ref() public {
        vm.expectRevert(LogicRefDenylist.ZeroLogicRefNotAllowed.selector, address(_denylist));
        _denylist.denyLogicRef(bytes32(0));
    }

    function test_listedLogicRefCount_counts_deprecated_and_denied_logic_refs() public {
        assertEq(_denylist.listedLogicRefCount(), 0, "the denylist should start empty");

        uint256 n = 10;
        for (uint256 i = 1; i < n; ++i) {
            if (i % 2 == 0) {
                _denylist.deprecateLogicRef(bytes32(i));
            } else {
                _denylist.denyLogicRef(bytes32(i));
            }
            assertEq(_denylist.listedLogicRefCount(), i, "the count should match the number of listed logic refs");
        }
    }

    function test_listedLogicRefAtIndex_returns_the_logic_refs_in_the_order_they_were_listed() public {
        uint256 n = 10;
        for (uint256 i = 0; i < n; ++i) {
            if (i % 2 == 0) {
                _denylist.deprecateLogicRef(bytes32(n - i));
            } else {
                _denylist.denyLogicRef(bytes32(n - i));
            }
        }

        for (uint256 i = 0; i < n; ++i) {
            assertEq(_denylist.listedLogicRefAtIndex(i), bytes32(n - i), "the logic ref at the index should match");
        }
    }

    function testFuzz_getLogicRefStatus_returns_active_for_an_unlisted_logic_ref(bytes32 logicRef) public view {
        _assertStatus(logicRef, ILogicRefDenylist.Status.Active, "an unlisted logic ref should be active");
    }

    function _assertStatus(bytes32 logicRef, ILogicRefDenylist.Status expected, string memory message) internal view {
        assertEq(uint8(_denylist.getLogicRefStatus(logicRef)), uint8(expected), message);
    }
}
