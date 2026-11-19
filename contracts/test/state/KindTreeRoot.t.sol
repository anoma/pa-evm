// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {IKindTreeRoot} from "../../src/interfaces/IKindTreeRoot.sol";
import {KindTreeRoot} from "../../src/state/KindTreeRoot.sol";
import {KindTreeRootMock} from "../mocks/KindTreeRoot.m.sol";

contract KindTreeRootTest is Test {
    KindTreeRootMock internal _kindTreeRoot;

    function setUp() public {
        _kindTreeRoot = new KindTreeRootMock();
    }

    function testFuzz_setKindTreeRoot_sets_the_root_and_emits_the_event(bytes32 newRoot) public {
        vm.assume(newRoot != bytes32(0));

        vm.expectEmit(address(_kindTreeRoot));
        emit IKindTreeRoot.KindTreeRootUpdated({kindTreeRoot: newRoot});
        _kindTreeRoot.setKindTreeRoot(newRoot);

        assertEq(_kindTreeRoot.getKindTreeRoot(), newRoot, "the root should be updated");
    }

    function test_setKindTreeRoot_reverts_on_the_zero_root() public {
        vm.expectRevert(KindTreeRoot.ZeroKindTreeRootNotAllowed.selector, address(_kindTreeRoot));
        _kindTreeRoot.setKindTreeRoot(bytes32(0));
    }

    function test_setKindTreeRoot_replaces_the_previous_root() public {
        _kindTreeRoot.setKindTreeRoot(bytes32(uint256(1)));
        _kindTreeRoot.setKindTreeRoot(bytes32(uint256(2)));

        assertEq(_kindTreeRoot.getKindTreeRoot(), bytes32(uint256(2)), "the latest root should be stored");
    }

    function testFuzz_isKindTreeRootAccepted_returns_true_for_the_stored_root(bytes32 storedRoot) public {
        vm.assume(storedRoot != bytes32(0));
        _kindTreeRoot.setKindTreeRoot(storedRoot);

        assertTrue(_kindTreeRoot.isKindTreeRootAccepted(storedRoot), "the stored root is accepted");
    }

    function testFuzz_isKindTreeRootAccepted_returns_true_for_the_empty_kind_tree(bytes32 storedRoot) public {
        vm.assume(storedRoot != bytes32(0));
        _kindTreeRoot.setKindTreeRoot(storedRoot);

        assertTrue(
            _kindTreeRoot.isKindTreeRootAccepted(_kindTreeRoot.EMPTY_KIND_TREE_ROOT()),
            "the empty kind tree is accepted"
        );
    }

    function testFuzz_isKindTreeRootAccepted_returns_false_for_any_other_root(bytes32 storedRoot, bytes32 otherRoot)
        public
    {
        vm.assume(storedRoot != bytes32(0));
        vm.assume(otherRoot != storedRoot && otherRoot != _kindTreeRoot.EMPTY_KIND_TREE_ROOT());
        _kindTreeRoot.setKindTreeRoot(storedRoot);

        assertFalse(_kindTreeRoot.isKindTreeRootAccepted(otherRoot), "any other root is rejected");
    }
}
