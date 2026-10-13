// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SlotDerivation} from "@openzeppelin-contracts-5.7.0/utils/SlotDerivation.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";

import {KindTableRootMock} from "../mocks/KindTableRoot.m.sol";

contract KindTableRootStorageTest is Test, KindTableRootMock {
    function test_storage_slot() public pure {
        assertEq(_KIND_TABLE_ROOT_STORAGE_SLOT, SlotDerivation.erc7201Slot("anoma.storage.KindTableRoot"));
    }

    function test_empty_kind_table_root_is_the_padding_leaf() public pure {
        assertEq(EMPTY_KIND_TABLE_ROOT, sha256("EMPTY"));
    }
}
