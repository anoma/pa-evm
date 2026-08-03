// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {reverseByteOrderUint32} from "risc0-risc0-ethereum-3.0.1/contracts/src/Util.sol";

import {Action, Consumed, Created, Transaction} from "../Types.sol";
import {Logic} from "./proving/Logic.sol";

/// @title RiscZeroUtils
/// @author Anoma Foundation, 2025
/// @notice A library encoding transactions to the RISC Zero journal of the aggregation instance — the risc0 serde
/// encoding of the arm `AggregationInstance` struct: `u32` values and element counts in little-endian byte order,
/// digests as raw 32 bytes.
/// @custom:security-contact security@anoma.foundation
library RiscZeroUtils {
    using RiscZeroUtils for Action;
    using RiscZeroUtils for Logic.AppData;

    /// @notice Converts a transaction to the RISC Zero journal of the aggregation instance.
    /// @param transaction The transaction to encode.
    /// @param complianceKey The compliance circuit verifying key to embed.
    /// @param kindTableCommitment The kind table commitment to embed.
    /// @return journal The resulting RISC Zero journal.
    /// @dev Element counts can safely be assumed to not exceed `type(uint32).max` as this would exceed Ethereum's
    /// block gas limit.
    function toJournal(Transaction calldata transaction, bytes32 complianceKey, bytes32 kindTableCommitment)
        internal
        pure
        returns (bytes memory journal)
    {
        uint256 actionCount = transaction.actions.length;

        journal = abi.encodePacked(
            complianceKey,
            kindTableCommitment,
            // forge-lint: disable-next-line(unsafe-typecast)
            reverseByteOrderUint32(uint32(actionCount))
        );

        for (uint256 i = 0; i < actionCount; ++i) {
            journal = abi.encodePacked(journal, transaction.actions[i].toJournal());
        }
    }

    /// @notice Converts an action to its part of the aggregation journal — the arm `ActionAggregated` encoding.
    /// @param action The action to encode.
    /// @return journal The resulting journal part.
    function toJournal(Action calldata action) internal pure returns (bytes memory journal) {
        uint256 consumedCount = action.consumed.length;
        // forge-lint: disable-next-line(unsafe-typecast)
        journal = abi.encodePacked(reverseByteOrderUint32(uint32(consumedCount)));

        for (uint256 i = 0; i < consumedCount; ++i) {
            Consumed calldata consumed = action.consumed[i];
            journal = abi.encodePacked(
                journal,
                consumed.nullifier,
                consumed.logicRef,
                consumed.commitmentTreeRoot,
                consumed.appData.toJournal()
            );
        }

        uint256 createdCount = action.created.length;
        // forge-lint: disable-next-line(unsafe-typecast)
        journal = abi.encodePacked(journal, reverseByteOrderUint32(uint32(createdCount)));

        for (uint256 i = 0; i < createdCount; ++i) {
            Created calldata created = action.created[i];
            journal = abi.encodePacked(journal, created.commitment, created.logicRef, created.appData.toJournal());
        }

        journal = abi.encodePacked(journal, bytes32(action.delta.x), bytes32(action.delta.y), action.actionTreeRoot);
    }

    /// @notice Converts application data to its journal encoding — the arm `AppData` encoding.
    /// @param appData The application data to encode.
    /// @return converted The resulting journal part.
    /// @dev Blob counts / payload lengths can safely be assumed to not exceed the `type(uint32).max` as this would
    /// exceed Ethereum's block gas limit.
    function toJournal(Logic.AppData calldata appData) internal pure returns (bytes memory converted) {
        converted = abi.encodePacked(
            // Encode the resource payload length as a `uint32` in reverse byte order.
            reverseByteOrderUint32(uint32(appData.resourcePayload.length)),
            encodePayload(appData.resourcePayload),
            //
            // Encode the discovery payload length as a `uint32` in reverse byte order.
            reverseByteOrderUint32(uint32(appData.discoveryPayload.length)),
            encodePayload(appData.discoveryPayload),
            //
            // Encode the external payload length as a `uint32` in reverse byte order.
            reverseByteOrderUint32(uint32(appData.externalPayload.length)),
            encodePayload(appData.externalPayload),
            //
            // Encode the application payload length as a `uint32` in reverse byte order.
            reverseByteOrderUint32(uint32(appData.applicationPayload.length)),
            encodePayload(appData.applicationPayload)
        );
    }

    /// @notice Encodes a payload to the RISC Zero journal format.
    /// @param payload The payload.
    /// @return encoded The encoded bytes of the payload.
    /// @dev The blob length divided by 4 can safely be assumed to not exceed the `type(uint32).max` as this
    /// would exceed Ethereum's block gas limit. Blobs are `Vec<u32>` values in arm, so a blob's byte length divided
    /// by 4 is its element count.
    function encodePayload(Logic.ExpirableBlob[] calldata payload) internal pure returns (bytes memory encoded) {
        uint256 blobCount = payload.length;
        for (uint256 i = 0; i < blobCount; ++i) {
            encoded = abi.encodePacked(
                encoded,
                // Encode the blob length (which is a multiple of `32 bytes`) divided by 4 (bytes) representing the
                // number of RISC Zero words in reverse (little-endian) byte order.
                reverseByteOrderUint32(uint32(payload[i].blob.length / 4)),
                payload[i].blob,
                // Encode the blob deletion criterion as a `uint32` in reverse (little-endian) byte order.
                reverseByteOrderUint32(uint32(payload[i].deletionCriterion))
            );
        }
    }
}
