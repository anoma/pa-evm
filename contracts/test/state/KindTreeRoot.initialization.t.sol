// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.7.0/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin-contracts-5.7.0/proxy/utils/Initializable.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";

import {IKindTreeRoot} from "../../src/interfaces/IKindTreeRoot.sol";
import {KindTreeRootMock} from "../mocks/KindTreeRoot.m.sol";

contract KindTreeRootInitializationTest is Test {
    KindTreeRootMock internal _kindTable;

    constructor() {
        _kindTable = _deployKindTreeRootMock();
    }

    function test_initialize_emits_the_KindTreeRootUpdated_event_for_the_empty_kind_table() public {
        bytes32 emptyKindTreeRoot = _kindTable.EMPTY_KIND_TREE_ROOT();

        vm.expectEmit();
        emit IKindTreeRoot.KindTreeRootUpdated({kindTreeRoot: emptyKindTreeRoot});
        _deployKindTreeRootMock();
    }

    function test_initialize_reverts_when_called_twice() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector, address(_kindTable));
        _kindTable.initialize();
    }

    function test_initialize_reverts_on_implementation_contract() public {
        KindTreeRootMock directMock = new KindTreeRootMock();

        vm.expectRevert(Initializable.InvalidInitialization.selector, address(directMock));
        directMock.initialize();
    }

    function test_initialize_sets_the_empty_kind_tree_root() public view {
        assertEq(
            _kindTable.getKindTreeRoot(),
            _kindTable.EMPTY_KIND_TREE_ROOT(),
            "the initial root should be the empty table's"
        );
    }

    /// @dev Deploys the mock behind an ERC-1967 proxy because the implementation contract disables the initializers.
    function _deployKindTreeRootMock() internal returns (KindTreeRootMock mock) {
        mock = KindTreeRootMock(
            address(new ERC1967Proxy(address(new KindTreeRootMock()), abi.encodeCall(KindTreeRootMock.initialize, ())))
        );
    }
}
