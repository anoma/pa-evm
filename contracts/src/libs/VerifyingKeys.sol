// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title VerifyingKeys
/// @author Anoma Foundation, 2025
/// @notice A library containing the verifying keys (RISC Zero image IDs) of the circuits the protocol adapter
/// accepts proofs from, pinned to the `anoma-rm-risc0` circuit binaries.
/// @custom:security-contact security@anoma.foundation
library VerifyingKeys {
    /// @notice The compliance circuit verifying key (`anoma-rm-risc0` `COMPLIANCE_VK`).
    bytes32 internal constant _COMPLIANCE = 0x88df64fe233c97307dd518c1757bf6cfca1f17f7103b4069dd9e2848db9d8434;

    /// @notice The batch aggregation circuit verifying key (`anoma-rm-risc0` `BATCH_AGGREGATION_EVM_VK`).
    bytes32 internal constant _BATCH_AGGREGATION = 0x4c0a771d29fce1983f108f4509552bb7f950c226f173f8d3244ef952dcde6978;
}
