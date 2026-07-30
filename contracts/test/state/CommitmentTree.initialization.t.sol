// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC1967Proxy} from "@openzeppelin-contracts-5.6.1/proxy/ERC1967/ERC1967Proxy.sol";
import {Initializable} from "@openzeppelin-contracts-5.6.1/proxy/utils/Initializable.sol";
import {Test} from "forge-std-1.16.1/src/Test.sol";

import {ICommitmentTree} from "../../src/interfaces/ICommitmentTree.sol";
import {SHA256} from "../../src/libs/SHA256.sol";
import {CommitmentTreeMock} from "../mocks/CommitmentTree.m.sol";

/// @dev The `initialize` under test belongs to the mock: `CommitmentTree` is abstract and exposes only the internal
/// `__CommitmentTree_init`, which inheritors call from their own initializer.
contract CommitmentTreeInitializationTest is Test {
    CommitmentTreeMock internal _cmAcc;

    constructor() {
        _cmAcc = _deployCommitmentTreeMock();
    }

    function test_initialize_stores_the_initial_root_being_the_empty_leaf_hash() public {
        CommitmentTreeMock newCmAcc = _deployCommitmentTreeMock();
        assertEq(newCmAcc.latestCommitmentTreeRoot(), SHA256.EMPTY_HASH, "The inital root should be the empty hash.");
        assertEq(newCmAcc.commitmentTreeRootCount(), 1, "The initial root count should be 1.");
    }

    function test_initialize_initializes_the_tree_with_depth_0() public {
        assertEq(_deployCommitmentTreeMock().commitmentTreeDepth(), 0, "The initial tree depth should be 0.");
    }

    function test_initialize_initializes_the_tree_with_capacity_1() public {
        assertEq(_deployCommitmentTreeMock().commitmentTreeCapacity(), 1, "The initial tree capacity should be 1.");
    }

    function test_initialize_initializes_the_tree_with_0_leaves() public {
        assertEq(_deployCommitmentTreeMock().commitmentCount(), 0, "The initial commitment count should be 0.");
    }

    function test_initialize_emits_the_CommitmentTreeRootAdded_event() public {
        address implementation = address(new CommitmentTreeMock());

        vm.expectEmit();
        emit ICommitmentTree.CommitmentTreeRootAdded({root: SHA256.EMPTY_HASH});
        new ERC1967Proxy(implementation, abi.encodeCall(CommitmentTreeMock.initialize, ()));
    }

    function test_initialize_reverts_when_called_twice() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector, address(_cmAcc));
        _cmAcc.initialize();
    }

    function test_initialize_reverts_on_implementation_contract() public {
        CommitmentTreeMock directMock = new CommitmentTreeMock();

        vm.expectRevert(Initializable.InvalidInitialization.selector, address(directMock));
        directMock.initialize();
    }

    /// @dev Deploys the mock behind an ERC-1967 proxy, initialized through the proxy constructor because the
    /// implementation contract disables the initializers.
    function _deployCommitmentTreeMock() internal returns (CommitmentTreeMock mock) {
        mock = CommitmentTreeMock(
            address(
                new ERC1967Proxy(address(new CommitmentTreeMock()), abi.encodeCall(CommitmentTreeMock.initialize, ()))
            )
        );
    }
}
