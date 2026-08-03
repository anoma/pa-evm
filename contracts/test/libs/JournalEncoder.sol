// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Logic} from "../../src/libs/proving/Logic.sol";
import {RiscZeroUtils} from "../../src/libs/RiscZeroUtils.sol";
import {Transaction} from "../../src/Types.sol";

/// @notice Re-exposes the calldata-only journal encoders behind an external boundary, letting tests encode
/// memory-built structures — the ABI encoding at the call boundary turns memory into calldata. External library
/// functions keep the boundary free of per-call deployments: forge links and deploys the library once.
library JournalEncoder {
    using RiscZeroUtils for Transaction;
    using RiscZeroUtils for Logic.AppData;

    function toJournal(Transaction calldata transaction, bytes32 complianceKey, bytes32 kindTableCommitment)
        external
        pure
        returns (bytes memory journal)
    {
        journal = transaction.toJournal({complianceKey: complianceKey, kindTableCommitment: kindTableCommitment});
    }

    function toJournal(Logic.AppData calldata appData) external pure returns (bytes memory journal) {
        journal = appData.toJournal();
    }
}
