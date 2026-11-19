// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SlotDerivation} from "@openzeppelin-contracts-5.7.0/utils/SlotDerivation.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";

import {KindTreeRootMock} from "../mocks/KindTreeRoot.m.sol";

contract KindTreeRootStorageTest is Test, KindTreeRootMock {
    function test_storage_slot() public pure {
        assertEq(_KIND_TREE_ROOT_STORAGE_SLOT, SlotDerivation.erc7201Slot("anoma.storage.KindTreeRoot"));
    }

    function test_empty_kind_tree_root_is_the_padding_leaf() public pure {
        assertEq(EMPTY_KIND_TREE_ROOT, sha256("EMPTY"));
    }
}
