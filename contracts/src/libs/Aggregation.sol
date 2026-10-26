// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IProtocolAdapter} from "../interfaces/IProtocolAdapter.sol";

/// @title Aggregation
/// @author Anoma Foundation, 2025
/// @notice A library computing the digest of the RISC Zero journal of the aggregation instance.
/// @custom:security-contact security@anoma.foundation
library Aggregation {
    /// @notice The size of the word that holds the length of a dynamic array or of `bytes` in front of its data.
    uint256 internal constant _LENGTH_WORD_SIZE = 0x20;

    /// @notice The offset of the actions in the journal, after its three head words: the compliance key, the kind tree
    /// root and this offset.
    uint256 internal constant _ACTIONS_OFFSET = 0x60;

    /// @notice Computes the SHA-256 digest of the RISC Zero journal of the aggregation instance,
    /// `abi.encode(complianceKey, kindTreeRoot, actions)`, with the kind tree root and the actions of a transaction.
    /// @dev The canonical encoding of the actions is the calldata from their length to the length of the delta proof,
    /// so the function hashes it in place. A transaction in another encoding fails the aggregation proof.
    /// @param transaction The transaction carrying the kind tree root and the actions.
    /// @param complianceKey The compliance circuit verifying key to embed.
    /// @return journalDigest The SHA-256 digest of the journal.
    function computeJournalDigest(IProtocolAdapter.Transaction calldata transaction, bytes32 complianceKey)
        internal
        pure
        returns (bytes32 journalDigest)
    {
        IProtocolAdapter.Action[] calldata actions = transaction.actions;
        bytes calldata deltaProof = transaction.deltaProof;
        uint256 start;
        uint256 end;
        // forge-lint: disable-next-item(inline-assembly)
        assembly {
            start := sub(actions.offset, _LENGTH_WORD_SIZE)
            end := sub(deltaProof.offset, _LENGTH_WORD_SIZE)
        }
        journalDigest =
            sha256(abi.encodePacked(complianceKey, transaction.kindTreeRoot, _ACTIONS_OFFSET, msg.data[start:end]));
    }
}
