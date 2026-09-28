// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SlotDerivation} from "@openzeppelin-contracts-5.7.0/utils/SlotDerivation.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";

import {KindTableCommitment} from "../../src/state/KindTableCommitment.sol";

contract KindTableCommitmentStorageTest is Test, KindTableCommitment {
    function test_storage_slot() public pure {
        assertEq(_KIND_TABLE_COMMITMENT_STORAGE_SLOT, SlotDerivation.erc7201Slot("anoma.storage.KindTableCommitment"));
    }

    function test_empty_kind_table_commitment_is_the_hash_of_empty_bytes() public pure {
        assertEq(_EMPTY_KIND_TABLE_COMMITMENT, sha256(""));
    }
}
