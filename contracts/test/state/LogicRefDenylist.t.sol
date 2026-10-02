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

    function testFuzz_denyLogicRef_denies_the_logic_ref(bytes32 logicRef) public {
        vm.assume(logicRef != bytes32(0));
        assertFalse(_denylist.isLogicRefDenied(logicRef), "the logic ref should not be denied before");

        _denylist.denyLogicRef(logicRef);

        assertTrue(_denylist.isLogicRefDenied(logicRef), "the logic ref should be denied after");
    }

    function test_denyLogicRef_emits_the_LogicRefDenied_event() public {
        vm.expectEmit(address(_denylist));
        emit ILogicRefDenylist.LogicRefDenied({logicRef: _EXAMPLE_LOGIC_REF});
        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF);
    }

    function test_denyLogicRef_reverts_on_duplicate() public {
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

    function test_deniedLogicRefCount_returns_the_count() public {
        assertEq(_denylist.deniedLogicRefCount(), 0, "the denylist should start empty");

        uint256 n = 10;
        for (uint256 i = 1; i < n; ++i) {
            _denylist.denyLogicRef(bytes32(i));
            assertEq(_denylist.deniedLogicRefCount(), i, "the count should match the number of denied logic refs");
        }
    }

    function test_deniedLogicRefAtIndex_returns_the_logic_refs_in_the_order_they_were_denied() public {
        uint256 n = 10;
        for (uint256 i = 0; i < n; ++i) {
            _denylist.denyLogicRef(bytes32(n - i));
        }

        for (uint256 i = 0; i < n; ++i) {
            assertEq(_denylist.deniedLogicRefAtIndex(i), bytes32(n - i), "the logic ref at the index should match");
        }
    }

    function test_isLogicRefDenied_returns_false_if_the_logic_ref_is_not_denied() public {
        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF);

        assertFalse(_denylist.isLogicRefDenied(bytes32(uint256(2))), "another logic ref should not be denied");
    }
}
