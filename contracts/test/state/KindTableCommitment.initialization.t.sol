// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";

import {IKindTableCommitment} from "../../src/interfaces/IKindTableCommitment.sol";
import {KindTableCommitmentMock} from "../mocks/KindTableCommitment.m.sol";

contract KindTableCommitmentInitializationTest is Test {
    KindTableCommitmentMock internal _kindTable;

    constructor() {
        _kindTable = _deployKindTableCommitmentMock();
    }

    function test_initialize_emits_the_KindTableCommitmentUpdated_event_for_the_empty_kind_table() public {
        bytes32 emptyKindTableCommitment = _kindTable.EMPTY_KIND_TABLE_COMMITMENT();

        vm.expectEmit();
        emit IKindTableCommitment.KindTableCommitmentUpdated({kindTableCommitment: emptyKindTableCommitment});
        _deployKindTableCommitmentMock();
    }

    function test_initialize_reverts_when_called_twice() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector, address(_kindTable));
        _kindTable.initialize();
    }

    function test_initialize_reverts_on_implementation_contract() public {
        KindTableCommitmentMock directMock = new KindTableCommitmentMock();

        vm.expectRevert(Initializable.InvalidInitialization.selector, address(directMock));
        directMock.initialize();
    }

    function test_initialize_sets_the_empty_kind_table_commitment() public view {
        assertEq(
            _kindTable.getKindTableCommitment(),
            _kindTable.EMPTY_KIND_TABLE_COMMITMENT(),
            "the initial commitment should be the empty table's"
        );
    }

    /// @dev Deploys the mock behind an ERC-1967 proxy because the implementation contract disables the initializers.
    function _deployKindTableCommitmentMock() internal returns (KindTableCommitmentMock mock) {
        mock = KindTableCommitmentMock(
            address(
                new ERC1967Proxy(
                    address(new KindTableCommitmentMock()), abi.encodeCall(KindTableCommitmentMock.initialize, ())
                )
            )
        );
    }
}
