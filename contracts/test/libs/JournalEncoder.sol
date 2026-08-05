// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IProtocolAdapter} from "../../src/interfaces/IProtocolAdapter.sol";
import {RiscZeroUtils} from "../../src/libs/RiscZeroUtils.sol";

/// @notice Re-exposes the calldata-only journal encoders behind an external boundary, letting tests encode
/// memory-built structures — the ABI encoding at the call boundary turns memory into calldata. External library
/// functions keep the boundary free of per-call deployments: forge links and deploys the library once.
library JournalEncoder {
    using RiscZeroUtils for IProtocolAdapter.Transaction;
    using RiscZeroUtils for IProtocolAdapter.AppData;

    function toJournal(
        IProtocolAdapter.Transaction calldata transaction,
        bytes32 complianceKey,
        bytes32 kindTableCommitment
    ) external pure returns (bytes memory journal) {
        journal = transaction.toJournal({complianceKey: complianceKey, kindTableCommitment: kindTableCommitment});
    }

    function toJournal(IProtocolAdapter.AppData calldata appData) external pure returns (bytes memory journal) {
        journal = appData.toJournal();
    }
}
