// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title VerifyingKeys
/// @author Anoma Foundation, 2026
/// @notice A library containing the verifying keys (RISC Zero image IDs) of the circuits the protocol adapter
/// accepts proofs from, pinned to the `anoma-rm-risc0` circuit binaries.
/// @custom:security-contact security@anoma.foundation
library VerifyingKeys {
    /// @notice The compliance circuit verifying key (`anoma-rm-risc0` `COMPLIANCE_VK`).
    bytes32 internal constant _COMPLIANCE = 0xbc4edaeea98f6501547cba34cf94d8b59631e7f86cf1a47119ba6f17c29a517c;

    /// @notice The batch aggregation circuit verifying key (`anoma-rm-risc0` `BATCH_AGGREGATION_EVM_VK`).
    bytes32 internal constant _BATCH_AGGREGATION_EVM =
        0x35082a39ecba805a1ca4fdac7eb364b20afce2abf14413ebd910233332ee45c9;
}
