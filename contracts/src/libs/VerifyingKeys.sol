// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/// @title VerifyingKeys
/// @author Anoma Foundation, 2025
/// @notice A library containing the verifying keys (RISC Zero image IDs) of the circuits the protocol adapter
/// accepts proofs from, pinned to the `anoma-rm-risc0` circuit binaries.
/// @custom:security-contact security@anoma.foundation
library VerifyingKeys {
    /// @notice The compliance circuit verifying key (`anoma-rm-risc0` `COMPLIANCE_VK`).
    bytes32 internal constant _COMPLIANCE = 0x7b657df4c7ee3ef8592894761aefc80f196e5b97dd27d43a98628b2ce2ef91f0;

    /// @notice The batch aggregation circuit verifying key (`anoma-rm-risc0` `BATCH_AGGREGATION_EVM_VK`).
    bytes32 internal constant _BATCH_AGGREGATION_EVM =
        0xa46d8bf487ebfdbe1d611a766b6a3fcb2884d2f226b3ce629f2bf25c411bce91;
}
