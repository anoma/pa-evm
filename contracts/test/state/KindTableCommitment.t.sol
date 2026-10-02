// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {IKindTableCommitment} from "../../src/interfaces/IKindTableCommitment.sol";
import {KindTableCommitment} from "../../src/state/KindTableCommitment.sol";
import {KindTableCommitmentMock} from "../mocks/KindTableCommitment.m.sol";

contract KindTableCommitmentTest is Test {
    KindTableCommitmentMock internal _kindTable;

    function setUp() public {
        _kindTable = new KindTableCommitmentMock();
    }

    function testFuzz_setKindTableCommitment_sets_the_commitment_and_emits_the_event(bytes32 newCommitment) public {
        vm.assume(newCommitment != bytes32(0));

        vm.expectEmit(address(_kindTable));
        emit IKindTableCommitment.KindTableCommitmentUpdated({kindTableCommitment: newCommitment});
        _kindTable.setKindTableCommitment(newCommitment);

        assertEq(_kindTable.getKindTableCommitment(), newCommitment, "the commitment should be updated");
    }

    function test_setKindTableCommitment_reverts_on_the_zero_commitment() public {
        vm.expectRevert(KindTableCommitment.ZeroKindTableCommitmentNotAllowed.selector, address(_kindTable));
        _kindTable.setKindTableCommitment(bytes32(0));
    }

    function test_setKindTableCommitment_replaces_the_previous_commitment() public {
        _kindTable.setKindTableCommitment(bytes32(uint256(1)));
        _kindTable.setKindTableCommitment(bytes32(uint256(2)));

        assertEq(_kindTable.getKindTableCommitment(), bytes32(uint256(2)), "the latest commitment should be stored");
    }

    function testFuzz_isKindTableCommitmentAccepted_returns_true_for_the_stored_commitment(bytes32 storedCommitment)
        public
    {
        vm.assume(storedCommitment != bytes32(0));
        _kindTable.setKindTableCommitment(storedCommitment);

        assertTrue(_kindTable.isKindTableCommitmentAccepted(storedCommitment), "the stored commitment is accepted");
    }

    function testFuzz_isKindTableCommitmentAccepted_returns_true_for_the_empty_kind_table(bytes32 storedCommitment)
        public
    {
        vm.assume(storedCommitment != bytes32(0));
        _kindTable.setKindTableCommitment(storedCommitment);

        assertTrue(
            _kindTable.isKindTableCommitmentAccepted(_kindTable.EMPTY_KIND_TABLE_COMMITMENT()),
            "the empty kind table is accepted"
        );
    }

    function testFuzz_isKindTableCommitmentAccepted_returns_false_for_any_other_commitment(
        bytes32 storedCommitment,
        bytes32 otherCommitment
    ) public {
        vm.assume(storedCommitment != bytes32(0));
        vm.assume(otherCommitment != storedCommitment && otherCommitment != _kindTable.EMPTY_KIND_TABLE_COMMITMENT());
        _kindTable.setKindTableCommitment(storedCommitment);

        assertFalse(_kindTable.isKindTableCommitmentAccepted(otherCommitment), "any other commitment is rejected");
    }
}
