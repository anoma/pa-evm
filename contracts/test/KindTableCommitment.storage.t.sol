// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {SlotDerivation} from "@openzeppelin-contracts-5.7.0/utils/SlotDerivation.sol";
import {Test} from "forge-std-1.16.2/src/Test.sol";

import {ProtocolAdapter} from "../src/ProtocolAdapter.sol";

contract KindTableCommitmentStorageTest is Test, ProtocolAdapter {
    constructor() ProtocolAdapter(address(1), bytes4(uint32(1))) {}

    function test_storage_slot() public pure {
        assertEq(_PROTOCOL_ADAPTER_STORAGE_SLOT, SlotDerivation.erc7201Slot("anoma.storage.ProtocolAdapter"));
    }

    function test_empty_kind_table_commitment_is_the_hash_of_empty_bytes() public pure {
        assertEq(_EMPTY_KIND_TABLE_COMMITMENT, sha256(""));
    }
}
