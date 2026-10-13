// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {IKindTableRoot} from "../../src/interfaces/IKindTableRoot.sol";
import {KindTableRoot} from "../../src/state/KindTableRoot.sol";
import {KindTableRootMock} from "../mocks/KindTableRoot.m.sol";

contract KindTableRootTest is Test {
    KindTableRootMock internal _kindTable;

    function setUp() public {
        _kindTable = new KindTableRootMock();
    }

    function testFuzz_setKindTableRoot_sets_the_root_and_emits_the_event(bytes32 newRoot) public {
        vm.assume(newRoot != bytes32(0));

        vm.expectEmit(address(_kindTable));
        emit IKindTableRoot.KindTableRootUpdated({kindTableRoot: newRoot});
        _kindTable.setKindTableRoot(newRoot);

        assertEq(_kindTable.getKindTableRoot(), newRoot, "the root should be updated");
    }

    function test_setKindTableRoot_reverts_on_the_zero_root() public {
        vm.expectRevert(KindTableRoot.ZeroKindTableRootNotAllowed.selector, address(_kindTable));
        _kindTable.setKindTableRoot(bytes32(0));
    }

    function test_setKindTableRoot_replaces_the_previous_root() public {
        _kindTable.setKindTableRoot(bytes32(uint256(1)));
        _kindTable.setKindTableRoot(bytes32(uint256(2)));

        assertEq(_kindTable.getKindTableRoot(), bytes32(uint256(2)), "the latest root should be stored");
    }

    function testFuzz_isKindTableRootAccepted_returns_true_for_the_stored_root(bytes32 storedRoot) public {
        vm.assume(storedRoot != bytes32(0));
        _kindTable.setKindTableRoot(storedRoot);

        assertTrue(_kindTable.isKindTableRootAccepted(storedRoot), "the stored root is accepted");
    }

    function testFuzz_isKindTableRootAccepted_returns_true_for_the_empty_kind_table(bytes32 storedRoot) public {
        vm.assume(storedRoot != bytes32(0));
        _kindTable.setKindTableRoot(storedRoot);

        assertTrue(
            _kindTable.isKindTableRootAccepted(_kindTable.EMPTY_KIND_TABLE_ROOT()), "the empty kind table is accepted"
        );
    }

    function testFuzz_isKindTableRootAccepted_returns_false_for_any_other_root(bytes32 storedRoot, bytes32 otherRoot)
        public
    {
        vm.assume(storedRoot != bytes32(0));
        vm.assume(otherRoot != storedRoot && otherRoot != _kindTable.EMPTY_KIND_TABLE_ROOT());
        _kindTable.setKindTableRoot(storedRoot);

        assertFalse(_kindTable.isKindTableRootAccepted(otherRoot), "any other root is rejected");
    }
}
