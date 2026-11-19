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

    function testFuzz_denyLogicRef_denies_the_logic_ref_in_one_denylist_only(bytes32 logicRef, bool consumed) public {
        vm.assume(logicRef != bytes32(0));
        assertFalse(_denylist.isLogicRefDenied(logicRef, consumed), "the logic ref should not be denied before");

        _denylist.denyLogicRef(logicRef, consumed);

        assertTrue(_denylist.isLogicRefDenied(logicRef, consumed), "the logic ref should be denied after");
        assertFalse(_denylist.isLogicRefDenied(logicRef, !consumed), "the other denylist should not deny it");
    }

    function testFuzz_denyLogicRef_keeps_the_logic_ref_in_the_other_denylist(bool consumed) public {
        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF, !consumed);

        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF, consumed);

        assertTrue(_denylist.isLogicRefDenied(_EXAMPLE_LOGIC_REF, consumed), "the logic ref should be denied after");
        assertTrue(_denylist.isLogicRefDenied(_EXAMPLE_LOGIC_REF, !consumed), "the other denylist should still deny it");
    }

    function testFuzz_denyLogicRef_emits_the_LogicRefDenied_event(bool consumed) public {
        vm.expectEmit(address(_denylist));
        emit ILogicRefDenylist.LogicRefDenied({logicRef: _EXAMPLE_LOGIC_REF, consumed: consumed});
        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF, consumed);
    }

    function testFuzz_denyLogicRef_reverts_on_duplicate(bool consumed) public {
        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF, consumed);

        vm.expectRevert(
            abi.encodeWithSelector(LogicRefDenylist.LogicRefAlreadyDenied.selector, _EXAMPLE_LOGIC_REF, consumed),
            address(_denylist)
        );
        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF, consumed);
    }

    function testFuzz_denyLogicRef_reverts_on_the_zero_logic_ref(bool consumed) public {
        vm.expectRevert(LogicRefDenylist.ZeroLogicRefNotAllowed.selector, address(_denylist));
        _denylist.denyLogicRef(bytes32(0), consumed);
    }

    function testFuzz_isLogicRefDenied_returns_false_if_the_logic_ref_is_not_denied(bool consumed) public {
        _denylist.denyLogicRef(_EXAMPLE_LOGIC_REF, consumed);

        assertFalse(_denylist.isLogicRefDenied(bytes32(uint256(2)), consumed), "another logic ref should not be denied");
    }

    function test_isLogicRefDenied_reads_the_same_slot_for_both_denylists() public {
        vm.record();
        _denylist.isLogicRefDenied(_EXAMPLE_LOGIC_REF, true);
        _denylist.isLogicRefDenied(_EXAMPLE_LOGIC_REF, false);
        (bytes32[] memory reads,) = vm.accesses(address(_denylist));

        assertEq(reads.length, 2, "each check should read one slot");
        assertEq(reads[0], reads[1], "both checks should read the same slot");
    }
}
