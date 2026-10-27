// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title VerifyingKeys
/// @author Anoma Foundation, 2026
/// @notice A library containing the verifying keys (RISC Zero image IDs) of the circuits the protocol adapter
/// accepts proofs from, pinned to the `anoma-rm-risc0` circuit binaries.
/// @custom:security-contact security@anoma.foundation
library VerifyingKeys {
    /// @notice The compliance circuit verifying key (`anoma-rm-risc0` `COMPLIANCE_VK`).
    bytes32 internal constant _COMPLIANCE = 0xbdb9cd903dbac3cd454a9cf7a145f5390b24f90d3fbc9420232339b7faf20be6;

    /// @notice The batch aggregation circuit verifying key (`anoma-rm-risc0` `BATCH_AGGREGATION_EVM_VK`).
    bytes32 internal constant _BATCH_AGGREGATION_EVM =
        0xecb83e9d3053617831ad6809c0ec018df7ee94f540892ceaa7eb5b62861d0bca;
}
