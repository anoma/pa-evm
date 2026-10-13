// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";

import {IKindTableRoot} from "../../src/interfaces/IKindTableRoot.sol";
import {KindTableRootMock} from "../mocks/KindTableRoot.m.sol";

contract KindTableRootInitializationTest is Test {
    KindTableRootMock internal _kindTable;

    constructor() {
        _kindTable = _deployKindTableRootMock();
    }

    function test_initialize_emits_the_KindTableRootUpdated_event_for_the_empty_kind_table() public {
        bytes32 emptyKindTableRoot = _kindTable.EMPTY_KIND_TABLE_ROOT();

        vm.expectEmit();
        emit IKindTableRoot.KindTableRootUpdated({kindTableRoot: emptyKindTableRoot});
        _deployKindTableRootMock();
    }

    function test_initialize_reverts_when_called_twice() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector, address(_kindTable));
        _kindTable.initialize();
    }

    function test_initialize_reverts_on_implementation_contract() public {
        KindTableRootMock directMock = new KindTableRootMock();

        vm.expectRevert(Initializable.InvalidInitialization.selector, address(directMock));
        directMock.initialize();
    }

    function test_initialize_sets_the_empty_kind_table_root() public view {
        assertEq(
            _kindTable.getKindTableRoot(),
            _kindTable.EMPTY_KIND_TABLE_ROOT(),
            "the initial root should be the empty table's"
        );
    }

    /// @dev Deploys the mock behind an ERC-1967 proxy because the implementation contract disables the initializers.
    function _deployKindTableRootMock() internal returns (KindTableRootMock mock) {
        mock = KindTableRootMock(
            address(
                new ERC1967Proxy(address(new KindTableRootMock()), abi.encodeCall(KindTableRootMock.initialize, ()))
            )
        );
    }
}
