// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.16.1/src/Test.sol";

import {Logic} from "../../src/libs/proving/Logic.sol";
import {RiscZeroUtils} from "../../src/libs/RiscZeroUtils.sol";

contract LogicProofTest is Test {
    using Logic for Logic.VerifierInput;
    using RiscZeroUtils for Logic.Instance;

    function testFuzz_different_empty_payloads_produce_different_digest(
        bytes32 root,
        bool consumed,
        bytes memory payload
    ) public pure {
        Logic.VerifierInput memory input;
        Logic.ExpirableBlob memory blob = Logic.ExpirableBlob(Logic.DeletionCriterion.Never, payload);

        Logic.ExpirableBlob[] memory payloadList = new Logic.ExpirableBlob[](1);
        payloadList[0] = blob;

        // Generate digest where only resource payload is filled.
        input.appData.resourcePayload = payloadList;
        bytes32 resourcePayloadDigest = sha256(input.toInstance(root, consumed).toJournal());
        assertEq(input.appData.resourcePayload.length, 1, "resourcePayload length should be 1");
        assertEq(input.appData.discoveryPayload.length, 0, "discoveryPayload length should be 0");
        assertEq(input.appData.externalPayload.length, 0, "externalPayload length should be 0");
        assertEq(input.appData.applicationPayload.length, 0, "applicationPayload length should be 0");
        input.appData.resourcePayload = new Logic.ExpirableBlob[](0);

        // Generate digest where only discovery payload is filled.
        input.appData.discoveryPayload = payloadList;
        bytes32 discoveryPayloadDigest = sha256(input.toInstance(root, consumed).toJournal());
        assertEq(input.appData.resourcePayload.length, 0, "resourcePayload length should be 0");
        assertEq(input.appData.discoveryPayload.length, 1, "discoveryPayload length should be 1");
        assertEq(input.appData.externalPayload.length, 0, "externalPayload length should be 0");
        assertEq(input.appData.applicationPayload.length, 0, "applicationPayload length should be 0");
        input.appData.discoveryPayload = new Logic.ExpirableBlob[](0);

        // Generate digest where only external payload is filled.
        input.appData.externalPayload = payloadList;
        bytes32 externalPayloadDigest = sha256(input.toInstance(root, consumed).toJournal());
        assertEq(input.appData.resourcePayload.length, 0, "resourcePayload length should be 0");
        assertEq(input.appData.discoveryPayload.length, 0, "discoveryPayload length should be 0");
        assertEq(input.appData.externalPayload.length, 1, "externalPayload length should be 1");
        assertEq(input.appData.applicationPayload.length, 0, "applicationPayload length should be 0");
        input.appData.externalPayload = new Logic.ExpirableBlob[](0);

        // Generate digest where only application payload is filled.
        input.appData.applicationPayload = payloadList;
        bytes32 applicationPayloadDigest = sha256(input.toInstance(root, consumed).toJournal());
        assertEq(input.appData.resourcePayload.length, 0, "resourcePayload length should be 0");
        assertEq(input.appData.discoveryPayload.length, 0, "discoveryPayload length should be 0");
        assertEq(input.appData.externalPayload.length, 0, "externalPayload length should be 0");
        assertEq(input.appData.applicationPayload.length, 1, "applicationPayload length should be 1");

        // Assert that all four produce different digests.
        assertTrue(resourcePayloadDigest != discoveryPayloadDigest, "resource and discovery digests should differ");
        assertTrue(resourcePayloadDigest != externalPayloadDigest, "resource and external digests should differ");
        assertTrue(resourcePayloadDigest != applicationPayloadDigest, "resource and application digests should differ");

        assertTrue(discoveryPayloadDigest != externalPayloadDigest, "discovery and external digests should differ");
        assertTrue(
            discoveryPayloadDigest != applicationPayloadDigest, "discovery and application digests should differ"
        );

        assertTrue(externalPayloadDigest != applicationPayloadDigest, "external and application digests should differ");
    }
}
