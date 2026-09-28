// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.16.2/src/Test.sol";

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
}
