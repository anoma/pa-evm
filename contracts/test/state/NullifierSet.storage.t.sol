// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SlotDerivation} from "@openzeppelin-contracts-5.7.0/utils/SlotDerivation.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";

import {NullifierSet} from "../../src/state/NullifierSet.sol";

contract NullifierSetStorageTest is Test, NullifierSet {
    function test_storage_slot() public pure {
        assertEq(_NULLIFIER_SET_STORAGE_SLOT, SlotDerivation.erc7201Slot("anoma.storage.NullifierSet"));
    }
}
