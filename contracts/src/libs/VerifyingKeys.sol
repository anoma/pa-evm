// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title VerifyingKeys
/// @author Anoma Foundation, 2026
/// @notice A library containing the verifying keys (RISC Zero image IDs) of the circuits the protocol adapter
/// accepts proofs from, pinned to the `anoma-rm-risc0` circuit binaries.
/// @custom:security-contact security@anoma.foundation
library VerifyingKeys {
    /// @notice The compliance circuit verifying key (`anoma-rm-risc0` `COMPLIANCE_VK`).
    bytes32 internal constant _COMPLIANCE = 0x406c60f87a5bb542a7fc7301ba5c01fe7724b5b3c9e335214d092a10b405f5a0;

    /// @notice The batch aggregation circuit verifying key (`anoma-rm-risc0` `BATCH_AGGREGATION_EVM_VK`).
    bytes32 internal constant _BATCH_AGGREGATION_EVM =
        0x858be23ecbd24b70efdfacada11c2f0471f33982e33758b8861e5d462576dc46;
}
