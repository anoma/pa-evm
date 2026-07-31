// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.16.1/src/Test.sol";

import {Logic} from "../../src/libs/proving/Logic.sol";
import {RiscZeroUtils} from "../../src/libs/RiscZeroUtils.sol";

contract AppDataJournalTest is Test {
    using RiscZeroUtils for Logic.AppData;

    /// @dev The four payload slots must be distinguishable in the journal — a blob moved to another slot must
    /// change the digest.
    function testFuzz_different_empty_payloads_produce_different_digest(bytes memory payload) public pure {
        Logic.AppData memory appData;
        Logic.ExpirableBlob memory blob = Logic.ExpirableBlob(Logic.DeletionCriterion.Never, payload);

        Logic.ExpirableBlob[] memory payloadList = new Logic.ExpirableBlob[](1);
        payloadList[0] = blob;

        Logic.ExpirableBlob[] memory emptyList = new Logic.ExpirableBlob[](0);
        appData.resourcePayload = emptyList;
        appData.discoveryPayload = emptyList;
        appData.externalPayload = emptyList;
        appData.applicationPayload = emptyList;

        // Generate digest where only the resource payload is filled.
        appData.resourcePayload = payloadList;
        bytes32 resourcePayloadDigest = sha256(appData.toJournal());
        appData.resourcePayload = emptyList;

        // Generate digest where only the discovery payload is filled.
        appData.discoveryPayload = payloadList;
        bytes32 discoveryPayloadDigest = sha256(appData.toJournal());
        appData.discoveryPayload = emptyList;

        // Generate digest where only the external payload is filled.
        appData.externalPayload = payloadList;
        bytes32 externalPayloadDigest = sha256(appData.toJournal());
        appData.externalPayload = emptyList;

        // Generate digest where only the application payload is filled.
        appData.applicationPayload = payloadList;
        bytes32 applicationPayloadDigest = sha256(appData.toJournal());

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
