// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.16.2/src/Test.sol";

import {Compliance} from "../../src/libs/proving/Compliance.sol";

contract ComplianceProofTest is Test {
    function testFuzz_compliance_instance_encoding(Compliance.Instance memory instance) public pure {
        assertEq(
            abi.encode(instance),
            abi.encodePacked(
                instance.consumed.nullifier,
                instance.consumed.logicRef,
                instance.consumed.commitmentTreeRoot,
                instance.created.commitment,
                instance.created.logicRef,
                instance.unitDeltaX,
                instance.unitDeltaY
            ),
            "compliance instance encoding should match packed encoding"
        );
    }
}
