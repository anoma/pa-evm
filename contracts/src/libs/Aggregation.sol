// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IProtocolAdapter} from "../interfaces/IProtocolAdapter.sol";

/// @title Aggregation
/// @author Anoma Foundation, 2025
/// @notice A library encoding the RISC Zero journal of the aggregation instance.
/// @custom:security-contact security@anoma.foundation
library Aggregation {
    /// @notice Converts the actions to the RISC Zero journal of the aggregation instance.
    /// @param actions The actions to encode.
    /// @param complianceKey The compliance circuit verifying key to embed.
    /// @param kindTableCommitment The kind table commitment to embed.
    /// @return journal The resulting RISC Zero journal.
    function toJournal(IProtocolAdapter.Action[] calldata actions, bytes32 complianceKey, bytes32 kindTableCommitment)
        internal
        pure
        returns (bytes memory journal)
    {
        journal = abi.encode(complianceKey, kindTableCommitment, actions);
    }
}
