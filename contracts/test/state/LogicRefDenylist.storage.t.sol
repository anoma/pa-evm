// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SlotDerivation} from "@openzeppelin-contracts-5.7.0/utils/SlotDerivation.sol";
import {Test} from "forge-std-1.17.0/src/Test.sol";

import {LogicRefDenylist} from "../../src/state/LogicRefDenylist.sol";

contract LogicRefDenylistStorageTest is Test, LogicRefDenylist {
    function test_storage_slot() public pure {
        assertEq(_LOGIC_REF_DENYLIST_STORAGE_SLOT, SlotDerivation.erc7201Slot("anoma.storage.LogicRefDenylist"));
    }
}
